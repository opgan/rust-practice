[![Clippy](https://github.com/opgan/rust-practice/actions/workflows/lint.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/lint.yml)
[![Tests](https://github.com/opgan/rust-practice/actions/workflows/main.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/main.yml)
[![Build binary release](https://github.com/opgan/rust-practice/actions/workflows/release.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/release.yml)

# RUST Project Scaffold
Getting Rust installed and creating new projects

##  Installing Rust
| Command | Description |
| ------- | ----------- |
| ``` curl https://sh.rustup.rs -sSf \| sh ``` | Download and install the latest stable version of Rust |
| ``` rustc --version ```| Check version |
| ``` rustup update ``` | Check update |
| ``` rustup self uninstall ```  | Uninstall Rust |

## Creating new project
| Command | Description |
| ------- | ----------- |
| ``` cargo new project_name ``` | Creating a Project with Cargo (use all-lowercase identifiers) |
| ``` tree . ```| Display the project structure (first, cd to project directory) |
| ``` rustup update ``` | Check update |
| ``` rustup self uninstall ```  | Uninstall Rust |

image
> 
.
├── Cargo.toml
└── src
    └── main.rs


* mv oldDIR newDIR
* Cargo.lock and Cargo.toml: name field must be same as directory name
* make test, lint, format, all



### Git commands used
| Command | Description |
| ------- | ----------- |
| web | https://github.com/opgan/Git-Commands/edit/master/README.md |
| `git status` | Check status |
| `git add [file-name.txt]` | Add a file to the staging area |
| `git add -A` | Add all new and changed files to the staging area |
| `git commit -m "[commit message]"` | Commit changes |
| `git stash` | Stash changes in a dirty working directory |
| `git push` | Push changes to remote repository (remembered branch) |
| `git pull` | Update local repository to the newest commit |
| `rm .git/index.lock` | Delete index.lock file in .git directory to remove another Git process running in repo |

### Git formatting
| Description |
| ----------- |
| https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#styling-text |
| **This is bold text** |
| ***All this text is important*** |
| This is a <sup>superscript</sup> text |
| This is an <ins>underlined</ins> text |
| > Text that is a quote |

