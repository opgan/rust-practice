[![Clippy](https://github.com/opgan/rust-practice/actions/workflows/lint.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/lint.yml)
[![Tests](https://github.com/opgan/rust-practice/actions/workflows/main.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/main.yml)
[![Build binary release](https://github.com/opgan/rust-practice/actions/workflows/release.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/release.yml)

# RUST Project Scaffold
Getting Rust installed and creating new projects

| Command | Description |
| ------- | ----------- |
| ``` curl https://sh.rustup.rs -sSf sh ``` | Install rust |
| ``` rustc --version ```| Check version |
| `git add -A` | Add all new and changed files to the staging area |
| `git commit -m "[commit message]"` | Commit changes |
| `git stash` | Stash changes in a dirty working directory |
| `git push` | Push changes to remote repository (remembered branch) |
| `git pull` | Update local repository to the newest commit |

##  Installing Rust
* run in terminal these commands
* installing command: ``` curl https://sh.rustup.rs -sSf | sh ```
* checking command for version and update: ``` rustc --version ``` ``` rustup update ```

## Creating new project
* creating: ``` cargo new project_name``` (note: Rust uses all-lowercase identifiers for functions and local variables)
* cd a_new_project
* tree .
.
├── Cargo.toml
└── src
    └── main.rs
Compiling:
rustc main.rs

* mv oldDIR newDIR
* Cargo.lock and Cargo.toml: name field must be same as directory name
* make test, lint, format, all
* git add *, git commit -m "remarks", git push


### Git commands used
| Command | Description |
| ------- | ----------- |
| `git status` | Check status |
| `git add [file-name.txt]` | Add a file to the staging area |
| `git add -A` | Add all new and changed files to the staging area |
| `git commit -m "[commit message]"` | Commit changes |
| `git stash` | Stash changes in a dirty working directory |
| `git push` | Push changes to remote repository (remembered branch) |
| `git pull` | Update local repository to the newest commit |


### Git formatting
REF: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#styling-text

**This is bold text**

***All this text is important***

This is a <sup>superscript</sup> text

This is an <ins>underlined</ins> text

> Text that is a quote

