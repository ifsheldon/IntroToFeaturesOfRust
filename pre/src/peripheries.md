# Peripheries

## `rustup`

`rustup` is the toolchain manager for Rust. You can use it to:

* Update everything that is officially support, including `rustc`, `cargo`, `rustdoc` and more
* Install different toolchains, like nightly, beta, stable versions
* Link a custom toolchain that you build yourself
* Set up toolchains for cross compilation
* and more, see [The rustup book](https://rust-lang.github.io/rustup/index.html).

## `cargo`

`cargo` is the project and dependency manager for Rust. 

Generally speaking, `rustc` is all you need to compile your code, but `cargo` makes dependency management, compilation and testing a lot easier. 

Consider it as a `maven` and a much more powerful `make` for Rust.

You can use `cargo` to:

* Manage dependencies of your project, see `Cargo.toml` and `Cargo.lock` in your project

* Build your project with different profiles (e.g. Release, Debug)

* Run tests, like `cargo test`

* Generate documentation web pages, like `cargo doc`

* Distribute and publish projects

* Format your code using `rustfmt` by `cargo fmt`

* Benchmark your code

* Install others’ binary tools

    Like `cargo install mdbook`, which installs `mdbook` for building this web-based “book”.

* Configure Continuous Integration (CI)

* and much more, see [The cargo book](https://doc.rust-lang.org/cargo/index.html)

