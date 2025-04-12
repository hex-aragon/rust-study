// Control flow 

fn main() {

    //control flow 
    let my_number = 5;
    let my_second_number = 10; 
    
    if my_number == 5 && my_second_number == 10 {
        println!("It's both match");
    } else if my_number == 6 { // && and || or 
        println!("It's six");
    } else {
        println!("It's a different number");
    }
    
    //match 
    
    let my_number2 : u8 = 5; 
    
     match my_number2 { //switch 
        0 => println!("It's a zero"), //
        1 => println!("It's a one"),
        _ => println!("It's a different number") // "I don't care" "anythinh else"
    }
    
        //expression-based language 
    let second_number = match my_number {
        0 => 23,
        1 => 65, 
        _ => 0 //
    };
    println!("The second number is : {}", second_number);
    
    
}