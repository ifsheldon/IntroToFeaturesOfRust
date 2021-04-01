use my_fancy_project::module_a::module_a_a;

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

fn main() {
    println!("Hello, world!");
    module_a_a::printFromModuleAA();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        println!("Hello test");
    }
}