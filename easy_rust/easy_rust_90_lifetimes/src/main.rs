//String
//&str 

//string literal 
//borrowed str 

// fn returns_reference<'a>() -> &'a str {
//         let my_string = "David".to_string(); //&'static 라이프 타임 - for the life of the program 
//         &my_string //&'a str - reference to something else 
// }

use std::fmt::Display;

struct Book<'a> { //Generics T, U
    name : &'a str 
}

fn returns_reference() -> &'static str {
        //let my_string = "David".to_string(); //&'static 라이프 타임 - for the life of the program 
        //&my_string //&'a str - reference to something else 
        "David"
}

fn print_thing<T: Display>(input: T){

}

fn main() { 
    
    let my_name = "David";
    println!("{my_name}");
  
    let my_book = Book {
        name: "my book"
    };

}
