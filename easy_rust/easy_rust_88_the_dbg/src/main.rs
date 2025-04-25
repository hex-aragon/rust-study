// dbg! // debug = quick print 

// AWS 
fn main() { 
    let mut my_number = dbg!(9); 
    dbg!(my_number);
    println!("{my_number}");
 
    dbg!();
 
    dbg!(10);
    println!("Something else");
 
    //기능별 체크 가능 
    let mut my_number2 = dbg!(9);
    dbg!(my_number2 += 10);
 
    let new_vec = dbg!(vec![8,9,10]);
 
    let double_vec = dbg!(new_vec 
         .iter()
         .map(|x| x * 2)
         .collect::<Vec<i32>>());
 }
 