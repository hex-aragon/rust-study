use std::collections::HashMap;
//pub fn entry(&mut self, key: K) -> Entry<K, V>
//macro_rules! {}
//
//

fn main() {
    //Key -> Value 
    // let mut book_hashmap = HashMap::new();
    
    // book_hashmap.insert(1, "L'Allemagne Moderne");
    
    // if let Some(book_name) = book_hashmap.get(&1) {
    //     println!("Already got a book : {}", book_name);
    // } else {
    //     book_hashmap.insert(1, "Le Petit Prince");
    // }
    
    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "섀도우 오브 유어 스마일",
        "Eye of the World",
        "Eye of the World",
    ]; //Eye of the World appears twice
    
    let mut book_hashmap = HashMap::new();
    // K: &str
    // V: i32
    
    for book in book_collection {
    //있으면 true, 없으면 false 
        //book_hashmap.entry(book).or_insert(true);
        //아래 변수는 mut 변수임 
        //없으면 데이터 적재 , 중복되면 +1
        let number_of_books = book_hashmap.entry(book).or_insert(0);
        *number_of_books += 1;
    }
    for (book, number) in book_hashmap {
        println!("{} : {} copies", book, number);
    }
    
    // for (book, true_or_false) in book_hashmap {
    //     println!("Do we have {} ? {}", book, true_or_false);
    // }
    
    
    
}
