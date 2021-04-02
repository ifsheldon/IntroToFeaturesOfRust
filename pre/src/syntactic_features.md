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
}

pub trait Hello {
    // immutable function
    fn get_name(&self) -> String;
    // mutable function
    fn change_name(&mut self, new_name: String);
    // function with default implementation
    fn say_hello(&self) {
        println!("Hello!");
    }
    // "static" function
    fn construct_from_name(name: String) -> Self;
}

impl Hello for People {
    fn get_name(&self) -> String { self.name.clone() }
    fn change_name(&mut self, new_name: String) { self.name = new_name; }
    fn say_hello(&self) { println!("Hi"); }
    // overriding the default
    fn construct_from_name(name: String) -> Self {
        People { name }
    }
}
```

## Closures

Closures, in many ways, are similar to lambdas in Java, but they are essentially close to lambdas in C++ because you also need to take care of the differences of capturing by value and capturing by reference, which are closely related to the lifetime mechanism(talked about later) in Rust.

The syntax is `|arguments| single-line expression` or `|arguments| {multiline expressions}` with the optional `move` keyword. 

For more details on closures, see [Chapter 13.1](https://doc.rust-lang.org/book/ch13-01-closures.html).

```rust
pub struct Integer(i32);
pub fn main(){
    let x = Integer(4);
    // in this closure, x is borrowed/ "captured" as reference
    let equal_to_x = |z: Integer| z.0 == x.0;
    let y = Integer(4);
    assert!(equal_to_x(y));
    // in below closure, x is moved/"captured" as data, thus owned by this closure, not owned by main() anymore
    let move_equal_to_x = move |z: Integer| z.0 == x.0;
    let z = Integer(4);
    assert!(!move_equal_to_x(z));
    println!("x = {}", x.0); // compile error here "value borrowed here after move"
}
```

## Generics

The simplest generics are just like those in Java.

```rust
struct Vec2<T>{
    x : T,
    y : T,
}
enum Option<T>{
    Some<T>,
    None
}
impl<T> Vec2<T>{
    fn x(&self)->&T{
        &self.x
    }
}
fn first<T, V>(binary_tuple: (T, V)) -> T {
    binary_tuple.0
}
```

However, we can constrain generics by traits, which is called trait bound, like

```rust
fn print<T: Display>(printable : T){
    println!("{}", printable);
}
```

For more on trait bounds, see [Chapter 10.2 Traits](https://doc.rust-lang.org/book/ch10-02-traits.html).

## Error Handling

Error handling in Rust requires no extra syntax, but just two `type`s, `Option<T>` and `Result<T, E>`.

For null values, `Option<T>` is used. Rust ensures **null-value safety** and saves you billions of dollars by enforcing strict type checking and the use of `Option<T>`. 

Because the type of `Option<T>` is not the same as `T`, you cannot pass type checking at compile time.

```c++
int* array_ptr = nullptr; // C++ is not null-value safe
```

```java
Object obj = null; // Java is not null-value safe
```

Definition of `Option<T>`:

```rust
pub enum Option<T>{
    Some(T), None
}
```

Example:

```rust
pub fn foo()->Option<i32>{
    Some(1)
}
pub fn bar()->Option<i32>{
    None
}
pub fn main(){
    // properly handles "null" value
    let result = foo();
    match result{
        Some(result_int)=>println!("Got some int {}", result_int),
        None=>println!("Nothing.")
    }
    // if you are sure
    let must_be_some = foo().unwrap();
    println!("Some int {}", must_be_some);
    let guess_wrong = bar().unwrap(); // runtime error, program exits because of underlying `panic!()` call.
}
```

For irrecoverable exceptions, use `panic!()`, like

``` rust
fn divide(x: i32, y: i32)-> i32{
    if y == 0{
        panic!("Divide by 0");
    }else{
        x / y
    }
}
```

For recoverable exceptions, `Result<T, E>` is used. The **error checking is mandatory** by enforcing type checking.

```java
static void method_throws_exception() throws Exception{
    throw new Exception();
}

public static main(String[] args){
    method_throws_exception(); // cannot compile because error checking is mandatory in Java
}
```

```c++
void func_throws_exception(){
    throw "Some exception occured"
}
int main(){
    func_throws_exception(); // can compile because c++ is indulging
}
```

Definition of `Result<T, E>`

```rust
pub enum Result<T, E>{
    Ok<T>, Err<E>
}
```

Example:

```rust
pub fn foo_result()->Result<i32, SomeExceptionType>{
    Ok(1)
}
pub fn bar_result()->Result<i32, SomeExceptionType>{
    Err(SomeExceptionType::new())
}

pub fn main(){
    // properly handles "null" value
    let result = foo_result();
    match result{
        Ok(result_int)=>println!("OK {}", result_int),
        Err(error)=>println!("Error {}", error)
    }
    // if you are sure
    let must_be_ok = foo_result().expect("I will never be wrong!");
    println!("Some int {}", must_be_ok);
    let wrong_again = bar_result().expect("It must be someone else messed up") // runtime error, program exits with stderr output because of underlying `panic!()` call
}
```

For more details, see [Chapter 9 Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html).