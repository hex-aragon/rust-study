//fn add_is_great(country_name: &mut String) {
    fn add_is_great(mut country_name: String) ->String { //take by value, declare as mutable
        country_name.push_str(" is great!");
        println!("Now it says: {}", country_name);
        country_name
    }
    
    fn main() {
       let my_country = "대한민국".to_string();
       // let mut my_country = "캐나다".to_string();
       //아래 함수가 소유권을 아예 가져감, 그래서 인수에서 mutable로 바뀜, 소유권자한테 
       //값이 들어왔기 때문 
         add_is_great( my_country); 
        // add_is_great(&mut my_country); 
    }
    