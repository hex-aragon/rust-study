//rust-playground monaco로 에디터 설정시 vs 코드와 유사해짐
#[derive(Debug)]
struct Book {
    title: String, 
    year: u16
}

fn main() {
    let my_book = Book {
        title: "Some title".to_string(),
        year: 1919
    };
    
    let book_2 = Book {
        title: "Book 2".to_string(),
        year: 2020
    };
   
   println!("Got books:\n{my_book:?}\n{book_2:?}");
   
   //padding = *
   //^ > 중앙 나오게끔 
   //? 디버그 프린트 
   println!("My book name: {my_book:*^16?}");

   let year = my_book.year; 
   println!("My book year : {} ", year);

   let width = 10;
   println!("My book name : {my_book:*^width$?}");

}