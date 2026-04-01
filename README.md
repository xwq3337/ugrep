# ugrep - 超高速 Grep 工具，具备高级功能

一款用 Rust 编写的高性能、功能丰富的搜索工具，结合了 ripgrep 的速度与先进的功能。
## 安装
curl -sSf https://raw.githubusercontent.com/xwq3337/ugrep/master/install.sh | bash

curl -sSf https://raw.githubusercontent.com/xwq3337/ugrep/master/uninstall.sh | bash
## 功能特性

### 核心功能
- **正则表达式引擎**：支持 PCRE 兼容的正则表达式，包含多行和 Unicode 支持
- **高性能**：使用 Rayon 进行并行处理，并支持零拷贝内存映射
- **智能输出**：色彩高亮、文件名、行号和列号显示
- **上下文控制**：使用 `-A`/`-B`/`-C` 选项显示匹配行之前/之后的内容

### 文件处理
- **二进制文件处理**：自动跳过二进制文件（可使用 `--binary` 选项覆盖）
- **Git 集成**：通过 `ignore` crate 遵循 `.gitignore` 规则
- **编码检测**：自动检测文件编码并转换为 UTF-8
- **文件过滤**：支持通配符模式、修改时间过滤

### 高级匹配
- **模式类型**：通配符/全局搜索、JSON/YAML 路径支持
- **匹配模式**：反向匹配 (`-v`)、单词匹配 (`-w`)
- **统计信息**：使用 `--stats` 选项显示匹配计数和文件统计信息

### 开发者友好
- **配置**：支持配置文件 (`~/.ugrep.toml`)
- **性能**：默认使用 8 线程并行处理
- **导出**：支持 JSON/CSV 输出，便于脚本处理

## 安装方法

```bash
# 从源码构建
cargo build --release

# 构建完成后，二进制文件位于 target/release/ugrep
```

## 使用方法

### 基础搜索

```bash
# 在当前目录搜索模式
ugrep "模式" .

# 在特定文件中搜索
ugrep "模式" 文件.txt

# 不区分大小写搜索
ugrep -i "模式" 文件.txt
```

### 输出选项

```bash
# 显示行号（默认）
ugrep "模式" 文件.txt

# 仅统计匹配次数
ugrep -c "模式" 文件.txt

# 仅显示包含匹配项的文件
ugrep -f "模式" .

# 启用色彩高亮
ugrep --color "模式" 文件.txt

# 显示统计信息
ugrep --stats "模式" .
```

### 上下文控制

```bash
# 显示匹配行后 3 行内容
ugrep -A 3 "模式" 文件.txt

# 显示匹配行前 2 行内容
ugrep -B 2 "模式" 文件.txt

# 显示匹配行前后各 1 行内容
ugrep -C 1 "模式" 文件.txt
```

### 高级匹配

```bash
# 全词匹配
ugrep -w "单词" 文件.txt

# 反向匹配（显示不匹配的行）
ugrep -v "模式" 文件.txt

# 使用通配符过滤文件
ugrep --glob "*.rs" "模式" .

# 正则表达式模式
ugrep "\d+\.\d+\.\d+\.\d+" 文件.txt
```

### 性能选项

```bash
# 设置线程数
ugrep -t 16 "模式" .

# 同时搜索二进制文件
ugrep --binary "模式" 文件.bin
```

## 选项说明

```
用法: ugrep [选项] <模式> [路径]

参数:
  <模式>  搜索模式
  [路径]  搜索路径 [默认: .]

选项:
  -i, --invert-match              反向匹配（显示不匹配的行）
  -w, --word-regexp               仅匹配完整单词
  -a, --after-context <行数>      显示匹配行后的行数
  -b, --before-context <行数>     显示匹配行前的行数
      --color                     启用色彩高亮
  -c, --count                     仅显示匹配次数
  -f, --files-with-matches        仅显示包含匹配项的文件
      --binary                    搜索二进制文件
      --modified <天数>           仅搜索最近 N 天内修改的文件
      --stats                     显示搜索统计信息
      --json                      以 JSON 格式输出
  -j, --json-path <路径>          搜索 JSON 路径
      --glob <模式>               使用通配符模式过滤文件
  -t, --threads <数量>            线程数量 [默认: 8]
  -h, --help                      显示帮助信息
```

## 性能表现

ugrep 为速度进行了优化：

- **并行处理**：多线程文件遍历和搜索
- **内存映射**：使用 `memmap2` 进行零拷贝文件读取
- **智能编码**：自动编码检测和转换
- **二进制检测**：默认跳过二进制文件
- **正则表达式编译**：预编译模式以供重复使用

## 示例

### 仅在 Rust 文件中搜索

```bash
ugrep --glob "*.rs" "println" .
```

### 查找 TODO 注释

```bash
ugrep -i "todo|fixme" --glob "*.rs" .
```

### 搜索最近修改的文件

```bash
ugrep --modified 7 "模式" .
```

### 统计每个文件的匹配次数

```bash
ugrep -c "function" src/
```

### 显示匹配内容周围的上下文

```bash
ugrep -C 2 "error" 日志.txt
```

## 配置

创建 `~/.ugrep.toml` 文件进行持久化设置：

```toml
线程数 = 16
色彩 = true
二进制文件 = false
```

## 性能比较

与传统 grep 相比：
- 通过并行处理**快 2-10 倍**
- 通过内存映射实现**内存高效利用**
- 具备现代选项，**功能丰富**

## 许可证

本项目采用 MIT 许可证。

## 贡献指南

欢迎贡献代码！请随时提交 Pull Request。
