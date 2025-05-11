# Ferrocyanide

![Rust][rust-image] 
[![🦀 Continuous Integration](https://github.com/jenskrumsieck/ferrocyanide/actions/workflows/build.yml/badge.svg)](https://github.com/jenskrumsieck/ferrocyanide/actions/workflows/build.yml)
![GitHub License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-green)
[![GitHub Release](https://img.shields.io/github/v/release/jenskrumsieck/ferrocyanide)](https://github.com/jenskrumsieck/ferrocyanide/releases/latest)
[![GitHub Downloads](https://img.shields.io/github/downloads/jenskrumsieck/ferrocyanide/total)]([https](https://github.com/jenskrumsieck/ferrocyanide/releases/latest))

Ferrocyanide is a small `proof-of-concept` Static-Site-Generator written in ![Rust][rust-image]. It started as a fun project, however a fully-functional Static-Site-Generator was produced which is used to build these pages.
_It supports two modes_:
- `serve`: Serves the website using a Rust Axum Server
- `build`: Builds static html pages

## Usage
```
Usage: ferrocyanide.exe <COMMAND>

Commands:
  serve  Serves the site in development mode
  build  Builds the site as static html files
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```


[rust-image]: https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white