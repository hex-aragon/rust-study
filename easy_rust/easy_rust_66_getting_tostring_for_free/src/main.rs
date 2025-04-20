trait PrintSomething {
    fn print_something(&self) where Self: std::fmt::Debug {
        //println!("I like to do stuff");
        println!("I am a :{:?}", self);
    }
}

trait PrintSomethingTwo {
    fn print_something_two(&self) where Self: std::fmt::Debug {
        //println!("I like to do stuff");
        println!("I am a 22 :{:?}", self);
    }
}

#[derive(Debug)]
struct Person;
#[derive(Debug)]
struct Building;

impl<T: std::fmt::Debug> PrintSomething for T {

} 

impl<T> PrintSomethingTwo for T {

}
// implement trait for all types that you want to have it 
// blanket trait implementation

fn main() {
    let person = Person; 
    let building = Building;
    person.print_something();
    building.print_something();

    println!("");
     let person = Person; 
    let building = Building;
    person.print_something_two();
    building.print_something_two();

}
