# Paradigmatic Features

## Favor Composition over Inheritance

There is **no** inheritance in Rust. The only inheritance-like thing you can do is defining a super trait that depends on other traits. 

Example

```rust
// if a type wants to implement HelloTwice, it must implement Hello first
trait HelloTwice : Hello{
    fn hello_twice(&self){
        self.say_hello();
        self.say_hello();
    }
}
trait FancyTrait : TraitA + TraitB + TraitC {
    fn fancy_func(&self);
}
```

In Java or C++, you can do inheritance like

```java
class Student extends People{} // Java inheritance
```

But in Rust, if you want to be as “inheritance" as possible (depending on your intention)

```rust
pub struct People{...}
impl Hello for People{...}
pub struct Student{
    people : People
}
// implement all traits People has for Student
impl Hello for Student{
    fn say_hello(&self){
        self.people.say_hello();
    }
}
```

## Favor Constructors over Default Values

Rust does NOT have:

* default values for functions
* functions with variable argument list
* function overloading

so every function in Rust is as explicit as possible.

To use defaults(like “constructors (in C++)” of a struct), you have to define different “constructors” with different name. Or, you can adopt the Constructor Pattern.

Example

```rust
pub struct Student{
    name: String,
    id: Option<u32>
}
// without Constructor Pattern
impl Student{
    pub fn new(name:String)->Self{
        Student{
            name,
            id:None
        }
    }
    pub fn new_with_id(name:String, id:u32)->Self{
        Student{
            name,
            id:Some(id)
        }
    }
}

//With Constuctor Pattern
pub struct StudentConstruct {
    pub student: Student
}

impl Student {
    pub fn construct(name: String) -> StudentConstruct {
        let student = Student { name, id: None };
        StudentConstruct { student }
    }
}

impl StudentConstruct {
    pub fn complete(self) -> Student {
        // why do I write `self` instead of `&self` here, BTW?
        self.student
    }
    pub fn add_id(&mut self, id: u32) -> &mut Self {
        self.student.id = Some(id);
        self
    }
}

pub fn main(){
    let student = Student::construct(String::from("mike"))
                                                .add_id(1)
                                                .complete();
}
```

## True Meta-programming with Macros

Why can `println!()` have one or more arguments while functions with variable argument list are not allowed?

Because it is a macro, not a defined function.

The macros in Rust basically has two types, one for preprocessing code(like `#[test]`), the other for generating code (like `println!()`). 

The decorative/annotation-like ones must be used with `#[]`.

The functional ones are always named with `!` appended, indicating they are macros.

We won’t go into details of macros, as they deserve a whole book. For usage details, see [Chapter 19.5](https://doc.rust-lang.org/book/ch19-06-macros.html), and for syntax details, see [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html).

An example of `println!()`

```rust
pub fn main(){
    println!("{} {} {} {}", 1, 2, 3, "?");
}
```

The expanded code (using `cargo-expand`) is 

```rust
pub fn main(){
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", " ", " ", " ", "\n"],
            &match (&1, &2, &3, &"?") {
                (arg0, arg1, arg2, arg3) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg3, ::core::fmt::Display::fmt),
                ],
            },
        ));
    }; // it is parsed to two arrays and passed to Arguments::new_v1()
}
```

The takeaway is with macros, you can program your program, bypassing syntactic constrains of Rust.