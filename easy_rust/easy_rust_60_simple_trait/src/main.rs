struct Animal {
    name: String
}

trait Canine {
    //dog-like 
    fn bark(&self){
        println!("Woof woof!");
    }
    fn run(&self){
        println!("I am running!");
    }
}

impl Canine for Animal {
        fn bark(&self){
        println!("멍멍!");
    }
}

fn main() {
    let my_animal = Animal {
        name: "Mr. Mantle".to_string()
    };
    
    my_animal.bark();
    my_animal.run();
}