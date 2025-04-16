

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
       None 
    } else {
        Some(value[4]) //i32,     
    }
    
}

//wrap in option
fn main() {

    let new_vec = vec![1,2,3,4,7,8,9,10];
    let index = take_fifth(new_vec); //Option<i32>
    println!("{:?}",index);
    
    let new_vec2 = vec![1, 2, 2];
    let index2 = take_fifth(new_vec2); // Some(1)
    //println!("{:?}",index2); 
    // .unwrap() - take out what is inside
//    println!("{}", index2.unwrap());
    
    
    // match index2 {
    //     Some(number) => println!("I got a number: {}", number),
    //     None => println!("There was nothing inside"),
    // }
    
    //.expect
    index2.expect("Needed at least five items - make sure Vec has at least five");
    
    // // Some(number)
    // if index2.is_some() { //bool 
    //     //Option<i32>
    //     println!("I got a number: {}", index2.unwrap());
    // }
    
}
