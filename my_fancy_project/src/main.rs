use my_fancy_project::module_a::module_a_a;
use rayon::prelude::*;
use std::fmt::Display;

///
/// The struct example
///
/// Fields are private, so instances cannot be constructed using simple struct syntax
///
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

pub trait IntroduceSelf {
    fn introduce_self(&self);
}

impl<T: Hello> IntroduceSelf for T {
    fn introduce_self(&self) {
        println!("Hello, I'm {}", self.get_name());
    }
}

fn first<T, V>(binary_tuple: (T, V)) -> T {
    binary_tuple.0
}

fn print<T: Display>(printable: T) {
    println!("{}", printable);
}

fn main() {
    println!("Hello, world!");
    module_a_a::printFromModuleAA();
    let data = vec![1, 2, 3, 4, 5];
    let double: Vec<i32> = data.iter().map(|x| 2 * x).collect();
    let parallel_double: Vec<i32> = data.par_iter().map(|x| 2 * x).collect();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        println!("Hello test");
    }

    pub struct Integer {
        pub int: i32
    }

    pub struct Data {
        pub integer: Integer
    }

    pub struct DataRef<'a> {
        pub data_ref: &'a Data
    }

    #[test]
    fn closures() {
        let x = Integer(4);
        let equal_to_x = |z: Integer| z.int == x.int;
        let y = Integer(4);
        let f = &mut y;
        assert!(equal_to_x(y));
        let move_equal_to_x = move |z: Integer| z.int == x.int;
        let z = Integer(4);
        assert!(!move_equal_to_x(z));
        println!("x = {}", x.0); // compile error here "value borrowed here after move"
    }

    fn mut_func1(data: Data) {
        data.integer.int = 4; // not allowed
    }

    fn mut_func2(mut data: Data) {
        data.integer.int = 4;
    }

    #[test]
    fn ownerships() {
        let integer = Integer { int: 1 };
        let data = Data { integer };
        mut_func2(data);
    }
}