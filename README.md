# 📊 CodeStat

A simple and powerful Node.js CLI tool to recursively analyze your project folder, count lines of code by file extension, and show code stats including empty lines, totals, and language usage percentages — all with color-coded terminal output.

---

## ✨ Features

- 🔁 Recursively scans all folders and files
- 🛑 Ignores folders using `.codestatignore` or default list
- 📦 Reports per-language:
  - Code lines
  - Empty lines
  - Total lines
  - % share of code lines
- 🎨 Beautiful, color-coded output
- 🌍 Usable globally with one `npm link`

---

## 📦 Installation

Clone this repo and install dependencies:

```bash
git clone https://github.com/your-username/codestat-js.git
cd codestat-js
npm install
```

Link it globally:

```bash
npm link
```

Now you can run `codestat` from anywhere.

---

## 🚀 Usage

From any project folder:

```bash
codestat
```

You'll see output like:

```
Code Statistics
27 files scanned.

js: 800 code lines, 200 empty lines (1000 total) - 55.56%
ts: 400 code lines, 100 empty lines (500 total) - 27.78%
json: 240 code lines, 10 empty lines (250 total) - 16.67%

Grand Total: 1750 lines
Code Only: 1440 lines
```

---

## 🧾 .codestatignore

You can create a `.codestatignore` file in the root of your project to exclude folders/files from scanning (one per line):

```txt
node_modules
dist
.git
coverage
build
```

If no `.codestatignore` is found, it defaults to ignoring:  
`node_modules`, `.git`, `dist`, `target`

---

## 🧠 How It Works

- Uses `fs` and `readline` to read files line-by-line.
- Recursively scans the directory tree.
- Groups and totals stats by file extension.
- Skips over any ignored folder at any depth.
- Uses `chalk` for clean terminal color output.

---

## 📄 License

MIT © [Your Name]