# Intro to Features Of Rust
This is an introduction to features of Rust for experienced Java/C++ programmers, which aims to build up an indexable comprehension of Rust even though technical details are omitted.
## Keywords for Rust
* Strictly typed
* Strongly static
* Concurrent safe
## Quick Rustup
* environment setup

    * Linux and macOS

        `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

    * Windows

        download `rustup-init.exe` on https://www.rust-lang.org/learn/get-started

    * Online playground(great for trying out but slow)

        https://play.rust-lang.org

* hello rust

    Simply `cargo new <path>`

    For example, `cargo new my_fancy_project` with default crate name as `my_fancy_project`

    Then `cd` and `cargo run`
## Syntactic Features
* Datatypes, `let` and static type inference
* Control Flows
* How to define a type ("class")
  * "method" and function definitions and restrictions
  * how to define a trait ("interface")
* Closures - Anonymous Functions
* Generics
* References and **lifetime**
* Enums and Pattern Matching
* Error handling

## Engineering Features
* Safe concurrency
* Null-value Safety
* Mandatory error checking
* Blanket implementation of traits
* No global variables
* `unsafe` keyword
* Full-fledge builtin testing and documentation support

## Paradigmatic Features
* Favor composition over inheritance
* Favor constructors over default values
* True meta-programming with macros 

## Peripharies
### `rustup`
toolchain management

### `cargo`
* dependency management
* building, publishing and deployment management
## Useful Links (sorted by importance)
* All-in-one official [official learning website](https://www.rust-lang.org/learn)
  * The book
  * Rust by examples
  * Core documentations
  * Domain specific references
    * webassambly book
  * Rustonomicon - the book of dark magics
  * Unstable book
* **Vibrant, active and friendly** community [forum](https://users.rust-lang.org/)
* My posts in the forum that attract many discussions
  * [how-to-implement-inheritance-like-feature-for-rust](https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159)
  * [is-there-a-simple-way-to-overload-functions](https://users.rust-lang.org/t/is-there-a-simple-way-to-overload-functions/30937)
* (Advertisement) My repo [RustyOOPatterns](https://github.com/ifsheldon/RustyOOPatterns)
