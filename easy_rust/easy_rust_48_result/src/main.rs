// Option의 형제 - Maybe there, maybe not 
// Result - May not work 
fn check_error(input: i32) -> Result<(), ()> {
    //Ok(())
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn check_error2(input: i32) -> Result<(), ()> {
    //Ok(())
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

//.is_some()
//.is_none()
// enum Option<T>{
//     None,
//     Some(T),
// }

// .is_ok()
// .is_err()
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    
    //let variable = check_error();
    if check_error2(6).is_ok() {
        println!("It's okay, guys! ");
    } else {
        println!("It's an error, guys!");
    }
    
    //option과 같은 효과 ? 
    //None.unwrap -> panic
    //Err.unwrap -> panic
    
    check_error(5).unwrap();
    
    // match check_error(5) {
    //     Ok(_) => println!("Okay guys"),
    //     Err(_) => println!("It's an error")  
    // }
    
}
