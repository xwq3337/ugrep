use anyhow::{Context, Result};
use clap::Parser;
use ignore::{Walk, WalkBuilder};
use memmap2::Mmap;
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(name = "ugrep")]
#[command(about = "A ultra fast grep with advanced features", long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    pub invert_match: bool,

    #[arg(short, long, default_value_t = false)]
    pub word_regexp: bool,

    #[arg(short, long, default_value_t = 0)]
    pub after_context: usize,

    #[arg(short, long, default_value_t = 0)]
    pub before_context: usize,

    #[arg(long, default_value_t = false)]
    pub color: bool,

    #[arg(short, long, default_value_t = false)]
    pub count: bool,

    #[arg(short, long, default_value_t = false)]
    pub files_with_matches: bool,

    #[arg(long, default_value_t = false)]
    pub binary: bool,

    #[arg(long)]
    pub modified: Option<usize>,

    #[arg(long, default_value_t = false)]
    pub stats: bool,

    #[arg(long, default_value_t = false)]
    pub json: bool,

    #[arg(short = 'j', long)]
    pub json_path: Option<String>,

    #[arg(long)]
    pub glob: Option<String>,

    #[arg(short, long, default_value_t = 8)]
    pub threads: usize,

    pub pattern: String,

    #[arg(default_value = ".")]
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Config {
    pub args: Args,
    pub pattern: Regex,
}

#[derive(Error, Debug)]
pub enum UGrepError {
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct MatchResult {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line: String,
    pub matches: Vec<(usize, usize)>,
}

pub struct SearchStats {
    pub files_searched: usize,
    pub files_matched: usize,
    pub total_matches: usize,
}

impl Config {
    pub fn build(args: std::env::Args) -> Result<Self> {
        let args = Args::try_parse_from(args)?;

        let pattern_str = if args.word_regexp {
            format!(r"\b{}\b", args.pattern)
        } else {
            args.pattern.clone()
        };

        let pattern = RegexBuilder::new(&pattern_str)
            .multi_line(true)
            .unicode(true)
            .build()
            .context("Failed to compile regex pattern")?;

        Ok(Config { args, pattern })
    }
}

pub fn run(config: Config) -> Result<()> {
    config.run()
}

impl Config {
    pub fn run(&self) -> Result<()> {
        let stats = SearchStats {
            files_searched: 0,
            files_matched: 0,
            total_matches: 0,
        };

        let walker = self.build_walker();

        if self.args.color {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            self.search_with_output(walker, &mut stdout, stats)
        } else {
            let mut stdout = StandardStream::stdout(ColorChoice::Never);
            self.search_with_output(walker, &mut stdout, stats)
        }
    }

    fn build_walker(&self) -> Walk {
        let mut builder = WalkBuilder::new(&self.args.path);
        builder.threads(self.args.threads);

        if let Some(glob_pattern) = &self.args.glob {
            let pattern = glob_pattern.clone();
            builder.filter_entry(move |entry| {
                entry
                    .file_name()
                    .to_str()
                    .map(|name| {
                        let regex_pattern = pattern.replace('*', ".*");
                        Regex::new(&regex_pattern)
                            .map(|re| re.is_match(name))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            });
        }

        builder.build()
    }

    fn search_with_output(
        &self,
        walker: Walk,
        stdout: &mut StandardStream,
        stats: SearchStats,
    ) -> Result<()> {
        let stats = Arc::new(Mutex::new(stats));
        let stdout = Arc::new(Mutex::new(stdout));

        walker
            .par_bridge()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_type()
                    .map(|ft| ft.is_file())
                    .unwrap_or(false)
            })
            .filter(|entry| {
                if let Some(days) = self.args.modified {
                    entry
                        .metadata()
                        .ok()
                        .and_then(|m| m.modified().ok())
                        .and_then(|modified| {
                            SystemTime::now()
                                .duration_since(modified)
                                .ok()
                                .map(|d| d.as_secs() / 86400 <= days as u64)
                        })
                        .unwrap_or(false)
                } else {
                    true
                }
            })
            .for_each(|entry| {
                let path = entry.path();
                if let Ok(results) = self.search_file(path) {
                    if !results.is_empty() {
                        let mut stats_guard = stats.lock().unwrap();
                        stats_guard.files_matched += 1;
                        stats_guard.total_matches += results.len();
                        drop(stats_guard);

                        if self.args.files_with_matches {
                            println!("{}", path.display());
                        } else if self.args.count {
                            println!("{}:{}", path.display(), results.len());
                        } else {
                            for result in results {
                                if let Ok(mut stdout_guard) = stdout.try_lock() {
                                    self.print_match(&mut stdout_guard, &result);
                                }
                            }
                        }
                    }
                    let mut stats_guard = stats.lock().unwrap();
                    stats_guard.files_searched += 1;
                }
            });

        if self.args.stats {
            let stats_guard = stats.lock().unwrap();
            eprintln!(
                "Files searched: {}, Files matched: {}, Total matches: {}",
                stats_guard.files_searched,
                stats_guard.files_matched,
                stats_guard.total_matches
            );
        }

        Ok(())
    }

    fn search_file(&self, path: &Path) -> Result<Vec<MatchResult>> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        if !self.args.binary && self.is_binary(&mmap) {
            return Ok(Vec::new());
        }

        let content = if let Ok(utf8_str) = std::str::from_utf8(&mmap) {
            utf8_str.to_string()
        } else {
            let mut detector = chardetng::EncodingDetector::new();
            detector.feed(&mmap, false);
            let encoding = detector.guess(None, true);
            
            let (decoded, _, _) = encoding.decode(&mmap);
            decoded.into_owned()
        };

        let mut results = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let matches: Vec<(usize, usize)> = self
                .pattern
                .find_iter(line)
                .map(|m| (m.start(), m.end()))
                .collect();

            let has_match = !matches.is_empty();
            let should_include = if self.args.invert_match {
                !has_match
            } else {
                has_match
            };

            if should_include {
                results.push(MatchResult {
                    file_path: path.to_path_buf(),
                    line_number: line_num + 1,
                    line: line.to_string(),
                    matches,
                });
            }
        }

        Ok(results)
    }

    fn is_binary(&self, data: &[u8]) -> bool {
        const MAX_CHECK: usize = 8192;
        let check_bytes = if data.len() > MAX_CHECK {
            &data[..MAX_CHECK]
        } else {
            data
        };

        check_bytes
            .iter()
            .filter(|&&b| b == 0)
            .take(1)
            .count()
            > 0
    }

    fn print_match(&self, stdout: &mut StandardStream, result: &MatchResult) {
        let prefix = format!("{}:{}:", result.file_path.display(), result.line_number);

        if self.args.color && !result.matches.is_empty() {
            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
            print!("{}", prefix);
            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
            println!("{}", self.highlight_line(&result.line, &result.matches));
            let _ = stdout.reset();
        } else {
            println!("{}{}", prefix, result.line);
        }
    }

    fn highlight_line(&self, line: &str, matches: &[(usize, usize)]) -> String {
        let mut result = String::new();
        let mut last_end = 0;

        for &(start, end) in matches {
            result.push_str(&line[last_end..start]);
            result.push_str("\x1b[1;31m"); // Bold red
            result.push_str(&line[start..end]);
            result.push_str("\x1b[0m"); // Reset
            last_end = end;
        }

        result.push_str(&line[last_end..]);
        result
    }
}
