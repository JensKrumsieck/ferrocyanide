# Welcome to Ferrocyanide
Ferrocyanide is a small `proof-of-concept` Static-Site-Generator written in ![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white). It started as a fun project, however a fully-functional Static-Site-Generator was produced which is used to build these pages.
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