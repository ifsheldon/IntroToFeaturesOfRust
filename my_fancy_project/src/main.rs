use my_fancy_project::module_a::module_a_a;
use rayon::prelude::*;
use std::fmt::Display;
use std::sync::{Mutex, Arc, MutexGuard};
use std::thread;

///
/// The struct example
///
/// Fields are private, so instances cannot be constructed using simple struct syntax
///
pub struct People {
    name: String,
}


pub struct Student {
    name: String,
    id: Option<u32>
}

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
        self.student
    }
    pub fn add_id(&mut self, id: u32) -> &mut Self {
        self.student.id = Some(id);
        self
    }
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

pub trait HelloTwice: Hello {
    fn hello_twice() {
        println!("s");
    }
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

const CONST_DATA: [u32; 4] = [1, 2, 3, 4];
const CONST_INT: i32 = 1;
static CONST_BOOL: bool = false;
// static IMMUT_STRING : String = String::from("aa");
static mut MUT_BOOL: bool = true;

fn main() {
    println!("Hello, world!");
    module_a_a::printFromModuleAA();
    let data = vec![1, 2, 3, 4, 5];
    let double: Vec<i32> = data.iter().map(|x| 2 * x).collect();
    let parallel_double: Vec<i32> = data.par_iter().map(|x| 2 * x).collect();
    println!("{}", CONST_BOOL);
    let some_bool = unsafe { MUT_BOOL };
    let numbers = vec![1, 2, 3, 4, 5];
    for i in 0..numbers.len() {
        let number_for_sure = unsafe { numbers.get_unchecked(i) };
        println!("Sure it is {}", number_for_sure);
    }

    let student = Student::construct(String::from("mike"))
                                                .add_id(1)
                                                .complete();
    let thread_num = 10;
    let mut handles = Vec::new();
    let values_created_in_threads: Vec<i32> = Vec::new();
    let guarded_values = Mutex::new(values_created_in_threads);
    let guarded_value_arc = Arc::new(guarded_values);
    for i in 0..thread_num {
        let arc_for_one_thread = Arc::clone(&guarded_value_arc);
        let thread_handle = thread::spawn(move || {
            let mut vec_guard = arc_for_one_thread.lock().unwrap(); // type is MutexGuard<Vec<i32>>
            vec_guard.push(i);
        });
        handles.push(thread_handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    for i in guarded_value_arc.lock().unwrap().iter() {
        println!("value : {}", i);
    }
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
        let x = Integer { int: 4 };
        let equal_to_x = |z: Integer| z.int == x.int;
        let y = Integer { int: 4 };
        ;
        // let f = &mut y;
        assert!(equal_to_x(y));
        let move_equal_to_x = move |z: Integer| z.int == x.int;
        let z = Integer { int: 3 };
        assert!(!move_equal_to_x(z));
        // println!("x = {}", x.int); // compile error here "value borrowed here after move"
    }

    fn mut_func1(data: Data) {
        // data.integer.int = 4; // not allowed
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