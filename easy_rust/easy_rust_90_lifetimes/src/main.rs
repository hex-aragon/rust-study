//String
//&str 

//string literal 
//borrowed str 

// fn returns_reference<'a>() -> &'a str {
//         let my_string = "David".to_string(); //&'static 라이프 타임 - for the life of the program 
//         &my_string //&'a str - reference to something else 
// }
// 위 함수는 잘못된 예시입니다. my_string은 함수가 끝나면 사라지므로, 그 참조를 반환하면 안 됩니다.

use std::fmt::Display; // Display 트레잇을 사용하기 위해 임포트

// Book 구조체 정의, name 필드는 라이프타임 'a를 가진 &str 참조
struct Book<'a> { //Generics T, U
    name : &'a str 
}

// 'static 라이프타임을 반환하는 함수, string literal은 프로그램 전체에서 살아있음
fn returns_reference() -> &'static str {
        //let my_string = "David".to_string(); //&'static 라이프 타임 - for the life of the program 
        //&my_string //&'a str - reference to something else 
        "David" // string literal 반환
}

// Display 트레잇을 구현한 타입을 받아들이는 함수, 실제로는 아무 동작도 하지 않음
fn print_thing<T: Display>(input: T){

}

fn main() { 
    
    let my_name = "David"; // string literal, &'static str 타입
    println!("{my_name}"); // my_name 출력
  
    let my_book = Book {
        name: "my book" // string literal을 name 필드에 할당
    };

}
