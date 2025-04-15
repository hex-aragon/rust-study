use std::fmt::{Display,Formatter, Result}; 

struct Book {
    year: i32 
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Book published in {}", self.year)
    }
}

fn give_thing<T: Display>(input: T) -> T { //T는 타입이란 뜻 ? 
    // let number = 9;
    // println!("The number is : {}", number);
    // number
    println!("{}", input); //Display 
    input
}

// generics - concreate
// i32 String 
// it's a little generic 

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    let z = give_thing(Book { year: 2025 } );
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}