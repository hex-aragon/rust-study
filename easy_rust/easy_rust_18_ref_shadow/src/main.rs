fn main() {
    
    let mut number = 10;
    // let number_ref = &number;
    // let number_change = &mut number;
    // *number_change += 10;
    // println!("{}", number_ref);
    
    
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("{}", number_ref);
    
    //shadowing 
    let country = "my country";
    let country_ref = &country;
    let country = 8;
    println!("{}", country);
    println!("{}", country_ref);
}