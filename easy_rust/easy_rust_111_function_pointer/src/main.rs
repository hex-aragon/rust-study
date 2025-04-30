fn gives_five() -> u8 {
    5
}

fn gives_six() -> u8 {
    6
}

// fn gives_seven() -> impl FnMut() {
//     7
// }

fn add_to_function_output(my_function: fn() -> u8, some_number : u8) {
    let my_number = my_function();
    let next_number = my_number + some_number;
    println!("We got {next_number}");
}

// closure
// Fn() -> u8 reference &Self 
// FnMut() -> u8 can mutate &mut Self 
// FnOnce() -> u8 can be used once Self 

fn main() {
    
    add_to_function_output(gives_five, 8);
    
    add_to_function_output(gives_six, 8);

}
