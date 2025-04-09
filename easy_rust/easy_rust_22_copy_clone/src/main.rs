fn prints_number(number: i32) {
    println!("{}", number);
}

fn prints_string(input: String){
    println!("{}", input);
}

//copy - copy types
//clone = non-copy types
fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);
        
    let my_country = "Austria".to_string();
    prints_string(my_country.clone()); //레퍼런스 귀찮으면 클론 쓰는 경우도 있음 
    prints_string(my_country.clone()); 
    prints_string(my_country);
}
