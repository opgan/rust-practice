[![Clippy](https://github.com/opgan/rust-practice/actions/workflows/lint.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/lint.yml)
[![Tests](https://github.com/opgan/rust-practice/actions/workflows/main.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/main.yml)
[![Build binary release](https://github.com/opgan/rust-practice/actions/workflows/release.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/release.yml)

# RUST Project Scaffold
Getting Rust installed and creating new projects

##  Installing Rust
* run in terminal these commands
* installing command: ``` curl https://sh.rustup.rs -sSf | sh ```
* checking command for version and update: ``` rustc --version ``` ``` rustup update ```

## Creating new project
* run in terminal these commands
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


