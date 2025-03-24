//Ownership
//move semantics 

// fn print_country(country_name: String) {
//     println!("My country is {}", country_name);
// }

fn print_country(country_name: &String) {
    println!("My country is {}", country_name);
    
}


fn main() {
    let  mut country = "대한민국".to_string();
    // print_country(country);
    // print_country(country); //소유권 에러 발생 
    // 아래는 rust스럽지 않음 
    // country = print_country(country);
    // country = print_country(country);
    // country = print_country(country);
    
    print_country(&country);
    print_country(&country);
    print_country(&country);
}
