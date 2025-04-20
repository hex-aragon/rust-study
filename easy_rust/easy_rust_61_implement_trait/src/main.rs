use std::fmt;


struct Animal {
    name: String
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8
}

//struct NotAnimal {}


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
        println!("멍멍! 나는 {}라고 한다", self.name);
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let age = self.age;
        write!(f, "My cat's name is {name}, and it is {age} years old")
    }
}


fn main() {
    let my_animal = Animal {
        name: "Mr. Mantle".to_string()
    };
    
    my_animal.bark();
    my_animal.run();


    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4
    };
    println!("{mr_mantle}");
    
}