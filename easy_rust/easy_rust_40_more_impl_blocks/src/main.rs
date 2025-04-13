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
    fn new_old_cat() -> Self{
        Self {
            age:15,
            animal_type : AnimalType::Cat
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
    
    fn new_dog(age: u8) -> Self { //function signature
    
        Self {
            age, 
            animal_type: AnimalType::Dog
        }
    }
    
    fn print(&self) {
        println!("I am a : {:?}", self);
    }
    
    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog! Now I am : {:?}", self);
    }  
    
     fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed to cat! Now I am : {:?}", self);
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
    
    
    let mut my_animal3 = Animal2::new_dog(15);
    println!("I made a : {:?}", my_animal3);
    my_animal3.print(); //dot operator
    
    Animal2::print(&my_animal3); //syntactic sugar 
    
    my_animal3.change_to_cat();
    my_animal3.change_to_dog();
    
    let my_old_cat = Animal2::new_old_cat();
    println!("my old cat: {:?}", my_old_cat);
}
