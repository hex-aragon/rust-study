//fold 
//.take_while
//.cloned
//.by_ref
//.skip_while
//.map_while

//lifetime
//.chunks
//.windows


use std::cmp::{max, min};

fn main() {
    let my_vec = vec![-878, 879879, -98798, 0, 76756];
    
    let biggest = my_vec.clone() //biggest
        .into_iter()
        .fold(i32::MIN,  |num1, num2 | 
            if num1 > num2 {
                num1 
            } else {
                num2 
            }
        );
    println!("Biggest is : {biggest}");
    
    let biggest2 = my_vec.clone() //biggest
        .into_iter()
        .fold(i32::MIN,  |num1, num2 | min(num1, num2));
        
    println!("Smallest is : {biggest2}");

    let biggest3 = my_vec.clone()
        .into_iter()
        .fold(i32::MIN,  |num1, num2 | max(num1, num2));
        
    println!("Smallest is : {biggest3}");


    let num_vec = vec![1,2,3,4,5,6,7,8,9,0];

    for chunk in num_vec.chunks(3) {
        println!("Chunk is : {:?}",chunk);
    }

    for window in num_vec.windows(3){
        println!("Window is : {window:?}");
    }
}
