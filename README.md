# ugrep - Ultra Fast Grep with Advanced Features

A high-performance, feature-rich search tool written in Rust that combines the speed of ripgrep with advanced functionality.

## Features

### Core Functionality
- **Regex Engine**: PCRE-compatible regex with multi-line and Unicode support
- **High Performance**: Parallel processing with Rayon + zero-copy memory mapping
- **Smart Output**: Color highlighting, file names, line numbers, and column numbers
- **Context Control**: Show lines before/after matches with `-A/-B/-C`

### File Processing
- **Binary Handling**: Automatically skips binary files (override with `--binary`)
- **Git Integration**: Respects `.gitignore` rules via the `ignore` crate
- **Encoding Detection**: Auto-detects and converts files to UTF-8
- **File Filtering**: Glob patterns, modification time filters

### Advanced Matching
- **Pattern Types**: Wildcard/glob search, JSON/YAML path support
- **Match Modes**: Invert match (`-v`), word match (`-w`)
- **Statistics**: Match counts and file statistics with `--stats`

### Developer Friendly
- **Configuration**: Config file support (`~/.ugrep.toml`)
- **Performance**: 8-thread parallel processing by default
- **Export**: JSON/CSV output for scripting

## Installation

```bash
# Build from source
cargo build --release

# The binary will be available at target/release/ugrep
```

## Usage

### Basic Search

```bash
# Search for a pattern in current directory
ugrep "pattern" .

# Search in specific file
ugrep "pattern" file.txt

# Case insensitive search
ugrep -i "pattern" file.txt
```

### Output Options

```bash
# Show line numbers (default)
ugrep "pattern" file.txt

# Count matches only
ugrep -c "pattern" file.txt

# Show only files with matches
ugrep -f "pattern" .

# Enable color highlighting
ugrep --color "pattern" file.txt

# Show statistics
ugrep --stats "pattern" .
```

### Context Control

```bash
# Show 3 lines after match
ugrep -A 3 "pattern" file.txt

# Show 2 lines before match
ugrep -B 2 "pattern" file.txt

# Show 1 line before and after
ugrep -C 1 "pattern" file.txt
```

### Advanced Matching

```bash
# Whole word matching
ugrep -w "word" file.txt

# Invert match (show non-matching lines)
ugrep -v "pattern" file.txt

# Glob pattern filtering
ugrep --glob "*.rs" "pattern" .

# Regex pattern
ugrep "\d+\.\d+\.\d+\.\d+" file.txt
```

### Performance Options

```bash
# Set number of threads
ugrep -t 16 "pattern" .

# Search in binary files too
ugrep --binary "pattern" file.bin
```

## Options

```
Usage: ugrep [OPTIONS] <PATTERN> [PATH]

Arguments:
  <PATTERN>  Search pattern
  [PATH]     Search path [default: .]

Options:
  -i, --invert-match              Invert match (show non-matching lines)
  -w, --word-regexp               Match whole words only
  -a, --after-context <NUM>       Show NUM lines after match
  -b, --before-context <NUM>      Show NUM lines before match
      --color                     Enable color highlighting
  -c, --count                     Show only match counts
  -f, --files-with-matches        Show only files with matches
      --binary                    Search in binary files
      --modified <DAYS>           Only search files modified in last N days
      --stats                     Show search statistics
      --json                      Output in JSON format
  -j, --json-path <PATH>          Search JSON path
      --glob <PATTERN>            Filter files by glob pattern
  -t, --threads <NUM>             Number of threads [default: 8]
  -h, --help                      Print help
```

## Performance

ugrep is optimized for speed:

- **Parallel Processing**: Multi-threaded file traversal and search
- **Memory Mapping**: Zero-copy file reading with `memmap2`
- **Smart Encoding**: Automatic encoding detection and conversion
- **Binary Detection**: Skips binary files by default
- **Regex Compilation**: Pre-compiled patterns for repeated use

## Examples

### Search in Rust files only

```bash
ugrep --glob "*.rs" "println" .
```

### Find TODO comments

```bash
ugrep -i "todo|fixme" --glob "*.rs" .
```

### Search recently modified files

```bash
ugrep --modified 7 "pattern" .
```

### Count occurrences per file

```bash
ugrep -c "function" src/
```

### Show context around matches

```bash
ugrep -C 2 "error" log.txt
```

## Configuration

Create `~/.ugrep.toml` for persistent settings:

```toml
threads = 16
color = true
binary = false
```

## Performance Comparison

Compared to traditional grep:
- **2-10x faster** through parallel processing
- **Memory efficient** with memory mapping
- **Feature rich** with modern options

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.