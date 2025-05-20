# 🚀 Codestat (Rust CLI Tool)

Codestat is a blazing fast Rust-based CLI tool to analyze your codebase by scanning directories recursively. It counts code and empty lines, groups by file extension, and displays language usage with colorful output.

---

## ✨ Features

- ⚡ Fast recursive directory traversal using `walkdir`
- 📂 Ignores folders via `.codestatignore` or default ignore list
- 🧮 Counts:
  - Code lines (non-empty)
  - Empty lines
  - Total lines
  - Per-file extension statistics
- 📊 Displays:
  - Per-language stats
  - Grand total lines
  - Code-only lines (excluding empty lines)
  - Language usage percentages
- 🎨 Color-coded terminal output using `colored`

---

## 🛠 Installation

Clone the project and build it:

```bash
git clone https://github.com/your-username/codestat-rust.git
cd codestat-rust
cargo build --release
```

To use it globally:

```bash
cargo install --path .
```

Now you can run the command from any directory containing source code:

```bash
codestat
```

---

## ⚙️ Usage

```bash
cd your/project/folder
codestat
```

Sample output:

```
Code Statistics
42 files scanned.

rs: 1200 code lines, 300 empty lines (1500 total) - 60.00%
js: 400 code lines, 100 empty lines (500 total) - 20.00%
html: 300 code lines, 50 empty lines (350 total) - 15.00%
json: 80 code lines, 10 empty lines (90 total) - 5.00%

Grand Total: 2440 lines
Code Only: 1980 lines
```

---

## 🚫 .codestatignore

Place a `.codestatignore` file in the root of your project to skip scanning specific folders:

```
node_modules
.git
dist
target
```

If no file is present, defaults are used.

---

## 🧠 How It Works

- Uses `walkdir` to walk the directory tree
- Skips ignored folders (recursively)
- Uses `BufReader` to read files line-by-line
- Categorizes lines and extensions
- Outputs summary and per-language breakdown

---

## 🧾 License

MIT © [Your Name]
