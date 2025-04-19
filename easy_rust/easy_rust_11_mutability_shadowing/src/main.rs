fn times_two(number: i32) -> i32 {
    number * 2 
}

fn main() {
    // let my_number = 8;
    let mut my_number = 8;

    my_number = 10; 
    println!("{}", my_number);
    
    let mut my_variable = 8; 
   // my_variable = "Hello, world!"; 
    let my_number2 = 8; // This is an i32
    println!("{}", my_number2); // prints 8
    let my_number2 = 9.2; // This is an f64 with the same name. But it's not the first my_number - it is completely different!
    println!("{}", my_number2); // Prints 9.2
    
    
    let final_number = {
        let y = 10;
        let x = 9;
        let x = times_two(x);
        let x = x + y; 
        x 
    };
    println!("The number is now: {}", final_number);
    
    
    let final_number2 = {
        let y = 13;
        let x = 9; // x starts at 9
        let x_twice = times_two(x); // second name for x
        let x_twice_and_y = x_twice + y; // third name for x!
        x_twice_and_y // too bad we didn't have shadowing - we could have just used x
    };
    println!("The number is now: {}", final_number2)
}