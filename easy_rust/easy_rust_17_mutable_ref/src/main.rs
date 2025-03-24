// & immutable reference / shared reference 
// &mut mutable reference / unique reference
// & * 

fn main() {
    let mut my_number = 9;
    //let num_ref = &mut my_number;
    //*num_ref = 10; 
    
    //reference 숫자 맞춰서 설정 
    let num_ref = &mut &mut my_number; 
    **num_ref = 10;
    
    
    
    println!("Number is now {}", my_number);
    
    
}