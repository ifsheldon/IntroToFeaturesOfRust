# Keywords of Rust

## Strictly Typed

|                                  |       C        |     C++      |                   Java                    | Rust |
| :------------------------------: | :------------: | :----------: | :---------------------------------------: | :--: |
|        Strong type system        |       No       |   Kind of    |                    Yes                    | Yes  |
|      Mutability Enforcement      |  Very limited  | Very limited |               Very limited                | Yes  |
| Distinguish between Ref and Data | Not applicable |     Weak     | All reference<br>(except primitive types) | Yes  |

In Rust, for mutability, we have:

* Immutable, which is the default
* `mut`, which must be stated explicitly

```rust,editable
struct Vec2{
    pub x : i32,
    pub y : i32,
}
fn main(){
    let i = 1;
    let ref_i = &i; // OK
    let mref_i = &mut i; // compile error, immutable data cannot have mutable ref
    i += 1; // compile error, i is immutable by default
    let mut j = 1;
    j += 1; // OK
    let vec = Vec2{ x:1, y:2};
    vec.x = 2; // compile error, vec is immutable
    let mut mvec = Vec2{x:1, y:2};
    mvec.x = 2; //OK
}
```

Data and their references are strictly distinguished, i.e. `Data != &Data` **semantically (owned data vs. borrowed data) and syntactically (leads to compile error)**.

```rust,editable
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

Related to **lifetime** that will be talked about later.

## Concurrent Safe

The compiler enforces concurrent safety by enforcing rules of **ownership and borrow checking**, which will be talked about later.

