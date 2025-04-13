// impl blocks 

#[derive(Debug)]
 struct Animal {
     age : u8,
     animal_type: AnimalType
}

#[derive(Debug)]
struct Animal2 {
     age : u8,
     animal_type: AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog
}

//impl = implement
impl Animal {
    fn new() -> Self { //function signature
    
        Self {
            age: 10, 
            animal_type: AnimalType::Cat
        }
    }
          
}

impl Animal2 {
    fn new_cat(age: u8) -> Self { //function signature
    
        Self {
            age, 
            animal_type: AnimalType::Cat
        }
    }
          
}
fn main() {
    
    let my_vec = vec![7,8];
    println!("The Vec length is: {}", my_vec.len()); //method
    
    // let my_animal = Animal {
    //     age: 10,
    //     animal_type: AnimalType::Cat
    // };
    let my_animal = Animal::new(); //associated function 
    //String::from("")
    println!("my_animal {:?} ",my_animal);
    
    
    
    let my_animal2 = Animal2::new_cat(10);
    println!("I made a : {:?}", my_animal2);
    
}
