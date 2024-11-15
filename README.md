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
| ``` cargo new project_name ``` | Creating a Project with Cargo  |
| ``` mv oldDIR newDIR ```  | Remaning directory (use all-lowercase identifiers) |
| ``` tree . ```| Display the project structure (first, cd to project directory) |
| ``` touch Makefile ``` | Create a make utility (Makefile) in project directory|
| ``` make format ```  | Formating codes |
| ``` make lint ```  |  Detect code mistakes |
| ``` make test ```  | Testing with assert |
| ``` make run ```  | Running project |
| ``` make release ```  | Creating an executable in target/release |

```
.
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    ├── debug
    │   ├── a_new__project
    │   ├── a_new__project.d
    │   ├── build
    │   ├── deps
    │   │   ├── a_new__project-1aa3bb9016acc2ae.d
    │   │   ├── a_new__project-3ded5645a512c160
    │   │   ├── a_new__project-3ded5645a512c160.d
    │   │   ├── a_new__project-bd9564115d356793.d
    │   │   ├── liba_new__project-1aa3bb9016acc2ae.rmeta
    │   │   └── liba_new__project-bd9564115d356793.rmeta
    │   ├── examples
    │   └── incremental
    │       ├── a_new__project-0mlp2tvocoh8w
    │       │   ├── s-h1tiag2ii6-0y8vg6s-2jo17jw9n6mqn4jnfv64sy7q3
    │       │   │   ├── dep-graph.bin
    │       │   │   ├── query-cache.bin
    │       │   │   └── work-products.bin
    │       │   └── s-h1tiag2ii6-0y8vg6s.lock
    │       ├── a_new__project-11krr1hp962ls
    │       │   ├── s-h1ti8fjofu-1h3tsoj-77aclmi19lfq87jz1n8kewkb6
    │       │   │   ├── 6trlwa3m9jc9a64ep6ci8jfdh.o
    │       │   │   ├── 7yt4tva7mgmo3ch4n74ohhgil.o
    │       │   │   ├── 8jd6gwzbfnt4qwxel4nl09g1n.o
    │       │   │   ├── 945fjiu4zsnieu6i2c5e50pkm.o
    │       │   │   ├── ahlf226onqazmyf7d1ref4urp.o
    │       │   │   ├── d35zkflpj2tqqlbhtyruevwg3.o
    │       │   │   ├── dep-graph.bin
    │       │   │   ├── query-cache.bin
    │       │   │   └── work-products.bin
    │       │   └── s-h1ti8fjofu-1h3tsoj.lock
    │       └── a_new__project-3c8lgibv4gjt4
    │           ├── s-h1ti192gdr-0vsb8i0-3xo7ozfduxm4m2waatjmye65e
    │           │   ├── dep-graph.bin
    │           │   ├── query-cache.bin
    │           │   └── work-products.bin
    │           └── s-h1ti192gdr-0vsb8i0.lock
    └── release
        ├── a_new__project
        ├── a_new__project.d
        ├── build
        ├── deps
        │   ├── a_new__project-873a7b852a93e9ef
        │   └── a_new__project-873a7b852a93e9ef.d
        ├── examples
        └── incremental
```

### Utility
* Makefile
```
rust-version:
	rustc --version
format:
	cargo fmt --quiet
lint:
	cargo clippy --quiet
test:
	cargo test --quiet
run:
	cargo run
release:
	cargo build --release
all: format lint test run
```

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

### Github workflows
* lint.yml
```
name: Clippy
on: [push, pull_request]
jobs: 
    build:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v1
            - uses: actions-rs/toolchain@v1
              with:
                    toolchain: stable
                    profile: minimal
                    components: clippy, rustfmt
                    override: true
            - name: Run clippy
              run: make lint
```
*tests.yml
```
name: Tests
on: [push, pull_request]
jobs: 
    build:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v1
            - uses: actions-rs/toolchain@v1
              with:
                    toolchain: stable
                    profile: minimal
                    components: clippy, rustfmt
                    override: true
            - name: Run clippy
              run: make test
```

* release.yml
```
name: Build binary release
on: [push, pull_request]
jobs: 
    build:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v1
            - uses: actions-rs/toolchain@v1
              with:
                    toolchain: stable
                    profile: minimal
                    components: clippy, rustfmt
                    override: true
            - name: Run clippy
              run: make release
```

### Git formatting
| Description |
| ----------- |
| https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#styling-text |
| **This is bold text** |
| ***All this text is important*** |
| This is a <sup>superscript</sup> text |
| This is an <ins>underlined</ins> text |
| > Text that is a quote |

