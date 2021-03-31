# Keywords of Rust

## Strictly Typed

|                                  |       C        |     C++      |                  Java                  | Rust |
| :------------------------------: | :------------: | :----------: | :------------------------------------: | :--: |
|        Strong type system        |       No       |   Kind of    |                  Yes                   | Yes  |
|      Mutability Enforcement      |  Very limited  | Very Limited |              Very Limited              | Yes  |
| Distinguish between Ref and Data | Not applicable |     Weak     | All reference(except basic data types) | Yes  |

In Rust, for mutability, we have:

* Immutable, which is the default
* `mut`, which must be stated explicitly

```rust
fn main(){
    let i = 1;
    i += 1; // compile error, i is immutable by default
    let mut j = 1;
    j += 1; // OK
}
```



Data and their references are strictly distinguished, i.e. `Data != &Data` **semantically (owned data vs. borrowed data) and syntactically (leads to compile error)**.

```rust
pub fn foo(reference : &i32){
    println!("{}", reference);
}

fn main() {
    let i = 1;
    foo(i); // Not allowed
    foo(&i); // OK
}
```

## Strongly Static

Related to **timeline** that will be talked about later.

## Concurrent Safe

The compiler enforces concurrent safety by enforcing rules of **ownership and borrow checking**, which will be talked about later.