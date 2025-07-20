# 📊 loc-counter

`loc-counter` is a fast and flexible CLI tool to count lines of code, comments, and blank lines in source code files across your project directories. It supports automatic language detection by file extension, multiple output formats, and folder exclusions.

---

## ✨ Features

- ✅ Automatically detects language from file extension
- ✅ Counts:
  - Total lines
  - Code lines
  - Comment lines
  - Blank lines
- ✅ Supports line and block comments
- ✅ Output formats: plain, JSON
- ✅ Optional file extension filtering (`--ext`)
- ✅ Exclude folders (like `target`, `node_modules`)
- ✅ Save JSON output to a file

---

## 📂 Supported File Types

| Language     | Extension | Line Comment | Block Comment     |
|--------------|-----------|--------------|-------------------|
| Rust         | `.rs`     | `//`         | `/* ... */`       |
| Python       | `.py`     | `#`          | `""" ... """`     |
| C++          | `.cpp`    | `//`         | `/* ... */`       |
| C            | `.c`      | `//`         | `/* ... */`       |
| JavaScript   | `.js`     | `//`         | `/* ... */`       |
| Java         | `.java`   | `//`         | `/* ... */`       |
| Shell        | `.sh`     | `#`          | *(none)*          |
| YAML         | `.yaml`   | `#`          | *(none)*          |


---

## 🚀 Run with Cargo (No Binary Needed)

```bash
cargo run -- --path ./src --format json --output stats.json --exclude target --ext cpp
```


| Option              | Description                                                          |
| ------------------- | -------------------------------------------------------------------- |
| `--path <DIR>`      | **(Required)** Path to directory or file to analyze                  |
| `--ext <EXT>`       | (Optional) Only analyze files with given extension (e.g. `rs`, `py`) |
| `--exclude <DIRS>`  | (Optional) Comma-separated folders to exclude (e.g. `target,tests`)  |
| `--format <FORMAT>` | Output format: `plain` (default), `json``                            |
| `--output <FILE>`   | (Optional) Path to save output (only for `--format json`)            |
| `--help`            | Show CLI help                                                        |
