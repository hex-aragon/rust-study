//use std::Option::*;

// Option
// Result
// OCaml 언어에서 러스트가 위의 개념을 가져옴 
// null

// enum Option<T> {
//     None,
//     Some(T),
// }

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
    let index = take_fifth(new_vec);
    println!("{:?}",index);
    
    let new_vec2 = vec![1,2,3];
    let index2 = take_fifth(new_vec2);
    println!("{:?}",index2);
}
