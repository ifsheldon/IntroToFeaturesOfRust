use my_fancy_project::module_a::module_a_a;

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