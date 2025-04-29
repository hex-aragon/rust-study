use std::fmt::Display;

//concrete 
fn print<T: Display>(input: T) {
    println!("print Hi, I'm a {input}");
}

//impl trait 
//concrete
fn print_2(input: impl Display) {
    println!("print_2 Hi, I'm a {input}");
}

// dynamic 
fn print_3(input : Box<dyn Display>) {
    println!("print_3 Hi  I'm  a {input}");
}

//i32 
fn print_i32(input : i32) {
    println!("print_i32 Hi  I'm  a {input}");
}

//string 
fn print_string(input : String) {
    println!("print_string Hi  I'm  a {input}");
}



fn main() {
    
    print(8);
    print_2(9);
    print_3(Box::new(12));
    print(String::from("I am a String"));

    print_i32(5);
    print_string("test".to_string());
}