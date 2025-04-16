use std::collections::HashMap;

fn main() {
    
    //Key -> Value
    //키 검색 > 밸류 출력
    
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];
    //Bielefeld 없는 도시라는 밈 같은 게 있음 
    //밈 유행 > ~>?
    
    let mut city_hashmap = HashMap::new();
    
    for city in canadian_cities {
        //key, value
        city_hashmap.insert(city, "Canada");
    }
    
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }
    
    //간단, 안전하지 않는거? 
    println!("{:?}", city_hashmap["Bielefeld"]);
    
    //안전
    println!("{:?}", city_hashmap.get("Bielefeld"));
    
    //일부러 에러 출력
    println!("{:?}", city_hashmap.get("Bielefeldd"));
    
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    // book_hashmap.insert(1, "Le Petit Prince");
    // book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    // book_hashmap.insert(1, "Eye of the World");
    
    if book_hashmap.get(&1).is_none() {
        book_hashmap.insert(1, "Le Petit Prince");
    } else {
        println!("Already got a book");
    }
    
    //에러 > println!("{:?}", book_hashmap.get(1));
    println!("{:?}", book_hashmap.get(&1));
    
    
    if let Some(book_name) = book_hashmap.get(&1) {
        println!("Already got a book: {}", book_name);
    } else {
        book_hashmap.insert(1, "Le Petit Prince");
    }
}
