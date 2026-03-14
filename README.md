# dw - Downloader CLI

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/version-0.3.4-blue" alt="Version">
  <img src="https://img.shields.io/badge/License-MIT-green" alt="License">
</p>

A blazing fast, simple command-line file downloader written in Rust with a beautiful progress bar.

## Features

- **Fast Downloads** - Built with Rust for maximum performance
- **Beautiful Progress Bar** - Visual feedback with customizable icons and colors
- **Resume Support** - Resume interrupted downloads seamlessly
- **Batch Downloads** - Download multiple files from a URL list
- **Customizable UI** - Personalize icons and colors for the progress bar
- **Quiet Mode** - Suppress output for scripting

## Installation

### From Source

```bash
git clone https://github.com/akbarahmedjonov/downloader-cli.git
cd downloader-cli
cargo build --release
sudo cp target/release/dw /usr/local/bin/
```

### Verify Installation

```bash
dw --version
```

## Usage

### Basic Download

```bash
dw https://example.com/file.zip
```

### Download to Specific Location

```bash
dw https://example.com/file.zip /path/to/save/file.zip
```

### Download to Directory

```bash
dw https://example.com/file.zip /path/to/directory/
```

### Options

| Flag | Description |
|------|-------------|
| `-f, --force` | Overwrite if the file already exists |
| `-c, --resume` | Resume a failed or cancelled download |
| `-e, --echo` | Print the filepath to stdout after downloading |
| `-q, --quiet` | Suppress filesize and progress info |
| `-b, --batch` | Download files in batch from a file with URLs |
| `-V, --version` | Show version information |
| `-h, --help` | Show help message |

### Custom Progress Bar

```bash
# Custom done/left icons
dw -f https://example.com/file.zip --done="#" --left="-"

# Custom border icons
dw -f https://example.com/file.zip --icon-border "[:]"

# Custom colors (black, red, green, yellow, blue, magenta, cyan, white)
dw -f https://example.com/file.zip --color-done green --color-left red

# All together
dw -f https://example.com/file.zip \
  --done "#" \
  --left "-" \
  --color-done green \
  --color-left red \
  --icon-border "[:]"
```

### Batch Downloads

Create a file with URLs (one per line):

```bash
# urls.txt
https://example.com/file1.zip
https://example.com/file2.zip
https://example.com/file3.zip
```

Then download all:

```bash
dw -b urls.txt /downloads/
```

### Resume Interrupted Download

```bash
dw -c https://example.com/large-file.zip
```

## Examples

```bash
# Simple download
dw https://github.com/akbarahmedjonov/downloader-cli/archive/refs/heads/master.zip

# Download with custom name
dw https://example.com/file.zip myfile.zip

# Quiet mode (useful in scripts)
dw -q https://example.com/file.zip /tmp/
# Output: /tmp/file.zip...success

# Echo filepath after download
dw -e https://example.com/file.zip /tmp/
# Output: /tmp/file.zip
```

## Requirements

- Rust 1.70+ (for building from source)
- Unix-like OS (Linux, macOS)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

**Akbar Ahmedjonov**
- GitHub: [@akbarahmedjonov](https://github.com/akbarahmedjonov)
- Email: akbarahmedjonovdev@gmail.com

---

<p align="center">
  Made with ❤️ in Rust
</p>
