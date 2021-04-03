# Ownership and Lifetime

Ownership and Lifetime are two major obstacles for people learning Rust.

> Notations (only here):
>
> ​	Data: a part of memory that is meaningful
>
> ​	Reference: a general pointer pointing to data
>
> ​	Procedure: a part of codes running on the same thread of the caller, could be a function
>
> ​	Subroutine: a function running on different threads from the caller

## Memory managements of C++/Java/Rust

|                          |        C++         | Java | Rust |
| :----------------------: | :----------------: | :--: | :--: |
|    Garbage Collection    |         No         | Yes  |  No  |
| Automatic Mem Management | NO, usually manual | Yes  | Yes  |

So, how can we do automatic memory management without GC overhead?

We need to find a way to determine when data are no longer needed:

* C++: (usually) when data are `free`d or `delete`d

* Java: When reference count to data is 0 (or only cyclic references)

* Rust: When data come to the end of **lifetime**.

## Ownership

If you are familiar with `unique_ptr` in C++, ownership in Rust is the same as the ownership in `unique_ptr`.

Who can own data in Rust:

* Functional owners:
    * Procedures
    * Subroutines
* Structural owners:
    * `struct`
    * `enum`

## Borrow & Lifetime

A variable is said to be borrowed if it is referenced.

Lifetime example:

```rust,editable
struct Data{
    pub name : String
}

fn function_owns(mut data: Data){
    data.name = String.from("new name");
    println!("{}", data.name);
    // the lifetime of `data` ends here
    // in this example, `data` is the same as `data1`
}

fn function_borrows(data:&Data){
    println!("{}", data.name);
}

pub fn main(){
    let data1 = Data{name: String::from("data 1")}; // data1 lifetime start here
    function_owns(data1); // data1 moved to functions_owns
    function_borrows(data1); // compile error
    function_owns(data1); // also compile error
    let data2 = Data{name : String::from("data 2")};
    function_borrows(data2);
    println!("data2 still alive {}", data2.name);
    // the lifetime of `data2` ends before re-assignment
    let data2 = Data{name : String::from("data 3")};
}
```

How about we storing a reference in a `struct`?

```rust,editable
pub struct BadDataRef{
	pub data_ref: &Data // compile error
}

pub fn main(){
    let data1 = Data{name: String::from("data 1")}; // data1 lifetime start here
    let bad_data_ref = BadDataRef{data_ref: &data1};
    println!("{}", bad_data_ref.data_ref.name); // OK
    function_owns(data1); // data1 moved to functions_owns
    println!("{}", bad_data_ref.data_ref.name); // classic dangling pointer problem!
}
```

We have to specify the lifetime of the referenced data somehow

```rust,editable
pub struct GoodDataRef<'a>{
    // 'a here is a lifetime specifier (and also a special kind of generics)
    // meaning the lifetime of referenced data is at least as long as the lifetimes of GoodDataRef instances
    pub data_ref: &'a Data
}

pub fn main(){
    let data1 = Data{name: String::from("data 1")}; // data1 lifetime start here
    let good_data_ref = GoodDataRef{data_ref: &data};
    println!("{}", good_data_ref.data_ref.name); // OK
    function_owns(data1); // data1 moved to functions_owns
    println!("{}", good_data_ref.data_ref.name); // compile error here, `good_data_ref` lives longer than `data1`
}
```

The borrow checker of Rust will always keep us safe from dangling pointers (unless you use `unsafe` code).

## More details

Please refer to [Chapter 4 Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) and [Chapter 10.3 Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).

