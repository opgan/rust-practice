[![Clippy](https://github.com/opgan/rust-practice/actions/workflows/lint.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/lint.yml)
[![Tests](https://github.com/opgan/rust-practice/actions/workflows/main.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/main.yml)
[![Build binary release](https://github.com/opgan/rust-practice/actions/workflows/release.yml/badge.svg)](https://github.com/opgan/rust-practice/actions/workflows/release.yml)

# Getting Started with RUST  
Getting Rust installed and creating new projects

##  Installing Rust
* https://www.rust-lang.org/tools/install

| Command | Description |
| ------- | ----------- |
| ``` curl https://sh.rustup.rs -sSf \| sh ``` | Download and install the latest stable version of Rust |
| ``` rustc --version ```| Check version |
| ``` rustup update ``` | Check update |
| ``` rustup self uninstall ```  | Uninstall Rust |

## Creating new project
| Command | Description |
| ------- | ----------- |
| ``` cargo new project_name ``` | Creating a Project with Cargo. Refer to project structure below. |
| ``` mv oldDIR newDIR ```  | Remaning directory (use all-lowercase identifiers). |
| ``` tree . ```| Display the project structure (first, cd to project directory) as shown below  |
| ``` touch Makefile ``` | Create a make utility (Makefile) in project directory as shown below |
| ``` make format ```  | Formating codes |
| ``` make lint ```  |  Detect code mistakes |
| ``` make test ```  | Testing with assert |
| ``` make run ```  | Running project |
| ``` make release ```  | Creating an executable in target/release |

* Rust project structure
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
    │   ├── examples
    │   └── incremental
    └── release
        ├── a_new__project
        ├── a_new__project.d
        ├── build
        ├── deps
        ├── examples
        └── incremental
```
* Makefile utility file defines a set of tasks (formatting, lint, test, run, etc) to be executed for build, tests, release and deploy as a good practice for better continuous integration and deployment (CI/CD).
* lint lint tool checks project source files for potential bugs and optimization improvements for correctness
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

### Updating Github repository (between local and remote environments)
https://github.com/opgan/Git-Commands/edit/master/README.md 
| Command | Description |
| ------- | ----------- |
| `git status` | Check status |
| `git add [file-name.txt]` | Add a file to the staging area |
| `git add -A` | Add all new and changed files to the staging area |
| `git commit -m "[commit message]"` | Commit changes |
| `git stash` | Stash changes in a dirty working directory |
| `git push` | Push changes to remote repository (remembered branch) |
| `git pull` | Update local repository to the newest commit |
| `rm .git/index.lock` | Delete index.lock file in .git directory to remove another Git process running in repo |

### Github action workflows
* https://docs.github.com/en/actions/writing-workflows/about-workflows
* Configurable automated process that will run one or more jobs. Defined by a YAML file in the repository and will run when triggered by an event in the repository
* Similar automation as in Makefile in local environment
```
$ tree .github/workflows/
.github/workflows/
├── lint.yml
├── main.yml
├── release.yml
└── tests.yml
```
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
https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#styling-text
| Description |
| ----------- |
| **This is bold text** |
| ***All this text is important*** |
| This is a <sup>superscript</sup> text |
| This is an <ins>underlined</ins> text |
| > Text that is a quote |

### Rust programming example project:
```
./a_new_project/
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── src
│   ├── lib.rs
│   ├── main.rs
│   └── my_mod.rs
└── target
```
* Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and manage visibility (public/private) between them.
* my_mod.rs and lib.rs can contain one or more functions
```
my_mod.rs

fn private_fun() {
    println!("called `my_mod::private_function()`");
}
// Use the `pub` modifier to override default visibility.
pub fn fun() {
    println!("called `my_mod::function()`");
    private_fun();
}
```
``` 
lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
* The declaration in main.rs will look for a file named `my_mod.rs` and will insert its contents inside a module named `my_mod` under main.rs scope
``` 
main.rs

mod my_mod;
use crate::my_mod::fun;
use a_new_project::add;

fn main() {
    println!("Hello, world!");
    println!("add(1,2)={}", add(1, 2));
    fun();
}
```

### Actix-Web framework practice example
```
./actix-web-practise/
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── src
│   └── main.rs
└── target
```

```
Cargo.toml

[package]
name = "actix-web-practise"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
```
```
main.rs
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```