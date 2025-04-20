//blanket trait implementations
//implementing a trait for every type that you want to have it 
use std::fmt::{Debug, Display};


trait Prints  {
    fn debug_print(&self) where Self: Debug {
        println!("I am : {:?}", self);
    }

    fn display_print(&self)  where Self: Display {
        println!("I am : {}", self);
    }

}

#[derive(Debug)]
struct Person;

#[derive(Debug)]
struct Building;

//impl Prints for Person {}
impl <T> Prints for T {
}

fn main() {
    let my_person = Person; 
    let my_building = Building; 
    my_person.debug_print();

    let x = String::from("hello");
    x.debug_print();
    x.display_print();
}
