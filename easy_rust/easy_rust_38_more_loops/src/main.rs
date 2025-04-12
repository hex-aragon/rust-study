
fn main() {

    let mut counter = 0;
    
    // loop {
    //     counter += 1;
    //     println!("The counter is now : {}", counter);
    //     if counter == 5 {
    //         //stop when counter == 5;
    //         break; 
    //     }
    // }
    while counter != 5 {
         counter += 1;
         println!("The counter is now : {}", counter);
    }
    
    //range 
    //for _ in 0..=3 { 
    //for _number in 0..=3 { 경고 없애려면 이렇게도 함  
    for number in 0..=3 { 
    //exclusive range 포함 안함 0..3
    //inclusive range 포함 함 0..=3 
        println!("The number is {}", number);
        //println!("I dont need a number")
    }
    
    let mut counter2 = 5;
    let my_number = loop {
        counter2 +=1;
        if counter2 % 53 == 3 {
           
            break counter2;
        }
        
    };
     println!("my_number is : {}",my_number)
    
}
