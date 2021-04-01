# Syntactic Features

## `let` and static type inference

Since Rust deploys static type inference, we don’t have to (always) specify a type for a variable.

```
let a = 1;
let b : u32 = 1;
```

but it’s always required that we specify a type for a constant

```rust
const c : u32 = 1;
```

## Data Types

For basic datatypes, we don’t have to memorize specifications any more, we have

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

`arch` means architecture dependent, and we will need `usize` to index into arrays.

### Full-fledged Unicode Support `char` and `String`

A `char` is 4-byte with comprehensive unicode support. 

Details of unicode support can be found in the book [Chapter 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings).

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let string = String::from(heart_eyed_cat);
}
```

### Tuples

```rust
fn main(){
    let tup = (500, 6.4, 1); // tuple composition
    let (x, y, z) = tup; // decomposition
}
```

### Arrays

```rust
fn main(){
    let a = [1,2,3,4,5]; // we use [] instead of {} in C/CPP
    let b : [u32; 5] = [1,2,3,4,5]; // specify type explicitly
    let c = [3; 5] // which is [3,3,3,3,3] of type [i32; 5]
    let d = [3_u32; 5] // which is [3,3,3,3,3] of type [u32; 5]
}
```

**Notice:**

* The length of an array must be strictly known at compile time, otherwise a compile error is thrown.

    ```rust
    fn main(){
        let len = 3;
        let arr_a = [3; len]; // compile error, even though len is immutable and known
        const const_len: usize = 3;
        let arr_b = [3; const_len]; // OK
    }
    ```
    
* The data of arrays are placed in static memory of a process(which is limited), so a very large array will cause runtime error.

    ```rust
    fn main(){
        let arr = [0; 100000000000000]; // compile OK, runtime error: SIGSEGV
    }
    ```

### Vec\<T\>

`Vec<T>` is the dynamic generic vector(similar to `vector` in C++), which is stored in the heap of a process.

```rust
fn main(){
    let v = Vec::new();
    for i in 0..100000000000000 {
        v.push(1); // totally OK at runtime
    }
}
```

## Functions and Return Value of an Expression

In a function, you can explicitly `return` a value, or, you can return the value of the last expression implicitly.

```rust
fn private_function(){
    // with no pub, a function is private by default
    // for more details, see https://doc.rust-lang.org/reference/visibility-and-privacy.html
}
pub fn foo()->i32 {
    1 // OK
}
pub fn bar()->i32 {
    return 1; // OK
}
pub fn foobar()->i32 {
    1; // compile error, but why?
}
```

The return value of `1` is not the same as `1;`

The former one return exactly `1` while `1;` returns `()`, which is of type `()`, not `i32`.

## Control Flows

* `if else`

    ```rust
    let i = 1;
    if i == 1 {
        println!("One");
    }else{
        println!("Not one");
    }
    // since if else block is also an expression
    let j = if i == 1 {
        1
    }else{
        0
    };
    ```

* `loop`, the infinite loop

    ```rust
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break with a expression return value
        }
    };
    ```

* `while` and `for in`

    ```rust
    let mut idx = 0;
    while idx < 5{
        idx += 1;
        println!("Some msg");
    }
    for i in 0..5{ // for in takes an iterator
        println!("Some msg");
    }
    ```
    
* `match` -- A much more powerful `switch` with pattern matching

    Notice: `match` does not need `break` and it **must be exhaustive**.

    For more about pattern matching, see [Chapter 18](https://doc.rust-lang.org/book/ch18-00-patterns.html)

    ```rust
    let i = 1;
    match i {
        1 => println!("one"),
        2 | 3 => println!("two or three"), // multiple values
        4..7 => println!("[4, 7)"), // exclusive range
        7..=9 => println!("[7, 9]"), // inclusive range
        _ => println!("else") // match anyway, compile error if delete this line
    }
    ```

    

## Type definitions

Rust has no “class”es and no inheritance, because it favors compositions over inheritance.

### Struct

Just as `struct` in C

```rust
pub struct Vec3{
    pub x : f32,
    pub y : f32,
    pub z : f32
} // again, the defaults are private

pub struct Temperature(f32); // Just a wrapper of a tuple, no runtime overhead

pub fn main(){
    let (x, y, z) = (1.0, 1.0, 1.0);
    let vec = Vec3{
        x, y, z // a syntactic sugar
    }
    let (xx, yy, zz) = (1.0, 1.0, 1.0);
    let vec1 = Vec3{
        x : xx,
        y : yy,
        z : zz
    } // Ordinary initialization
    let temperature = Temperature(30.0);
    println!("Temperature is {} C", temperature.0)
}
```

#### Binding functions — “methods”

The “methods” of a Rust `struct` are not methods but just functions bound to a `struct`.

```rust
impl Vec3{
    pub fn squred_length(&self) -> f32{
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            x, y, z
        }
    }
}
```

What's the difference between a function with `&self` and one without it? **No difference!**

```rust
fn main() {
    let v = Vec3::new(1.0,1.0,1.0);
    // the below two lines are equivalent
    println!("Squared length is {}", Vec3::squared_length(&v));
    println!("Squared length is {}", v.squared_length()); // more like a syntactic sugar
}
```

### Enum

With pattern matching, `enum` in Rust is much more powerful than `enum` in Java/C++.

```rust
// example
pub enum SimpleResult{
    OK(i32), Err(i32)
}
pub fn main(){
    let ok30 = SimpleResult::OK(30);
    match ok30 {
        SimpleResult::OK(code) => println!("OK {}", code),
        SimpleResult::Err(err_code) => eprintln!("Err code {}", err_code)
    }
    // match as an expression
    let code = match ok30{
        SimpleResult::OK(code) => code,
        SimpleResult::Err(err_code) => err_code
    };
}
```

We will see enums `Option<T>` and `Result<T, E>` in error handling.

### Trait — The “interface”

The “Interface” in Rust is `trait`, which is a limited version of Java’s “Interface” because it can only specify functions that are needed to implement the trait, but **cannot** specify fields.

For more details on traits, see [Chapter 10.2 Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) and [Chapter 19.3 Advance Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html).

```rust
pub struct People {
    name: String,
    age: u8,
}

pub trait Hello {
    // immutable function
    fn give_age(&self) -> u8;
    fn get_name(&self) -> String;
    // mutable function
    fn change_name(&mut self);
    // function with default implementation
    fn say_hello(&self) {
        println!("Hello!");
    }
    // "static" function
    fn construct_from_name(name: String) -> Self;
}

// impl Trait for Type
impl Hello for People {
    fn get_name(&self) -> String { self.name.clone() }
    fn change_name(&mut self, new_name: String) { self.name = new_name; }
    fn say_hello(&self) { println!("Hi"); } // overriding the default
    fn construct_from_name(name: String) -> Self {
        People { name, age: 0 }
    }
}
```

#### Blanket Trait Implementation

Blanket Implementation is closely related to generics.

```rust
impl<T: Hello> IntroduceSelf for T {
    fn introduce_self(&self) {
        println!("Hello, I'm {}", self.get_name());
    }
}
```

It is much more powerful in a magical way, see the `Rayon` example:

```rust
pub fn main(){
    let data = Vector::new()
    for i in 0..10000000000000{
        data.push(i);
    }
}
```





