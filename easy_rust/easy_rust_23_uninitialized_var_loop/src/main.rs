fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0{ // 102 / 50 2 remainder 2 
        // 50 >> 
                break; 
        }
    }
    counter
}

// uninitialized variable
// control flow
// possible uninitialized = maybe doesn't have a value yet
fn main() {
    println!("Hello World!");
    
    
    //let my_number: u8; 
    
    let my_number;
    
    {
        //복잡한 것들 
        // let x = 9;
        //    x + 9
        let x = loop_then_return(43); //50 
        my_number = x //50 
        //my_number = 9;
    };
    
    println!("{}", my_number); // >> 50  
    
    
}
