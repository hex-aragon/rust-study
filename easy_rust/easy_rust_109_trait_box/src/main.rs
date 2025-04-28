//recursive types (재귀적 타입)
//owned data (소유권이 있는 데이터)

// 재귀적 열거형 정의
enum List {
    //content: List >> error : recursive type `List` has infinite size
    //직접 List를 포함하면 크기를 컴파일러가 계산할 수 없어 에러 발생
    
    //content: Box<List>
    // Box를 사용하면 힙에 할당되어 크기가 고정됨
    Content(i32, Box<List>), // i32 값과 다음 List를 가리키는 Box 포함
    NoContent               // 리스트의 끝을 나타내는 variant
}

// Booky 트레잇 정의 (빈 트레잇)
trait Booky {}

// Book 구조체 정의
struct Book {
    name: String
}

// BigBook 구조체 정의 (필드 없음)
struct BigBook;

// City 구조체 정의
struct City {
    year_founded: i32 
}

// 각 구조체에 Booky 트레잇 구현
impl Booky for Book{}
impl Booky for BigBook{}
impl Booky for City{}

fn main() {
    // 잘못된 리스트 생성 예시 (주석 처리됨)
    // let my_list = List {
    //    // content: Box::new(List)
    // }

    // 올바른 리스트 생성
    // 8786을 값으로 가지고, 그 다음은 NoContent인 리스트 생성
    let my_list = List::Content(8786, Box::new(List::NoContent));

    // City 인스턴스 생성
    let my_city = City {
        year_founded: 1989
    };
    
    //dyn 
    //dynamically decide : 런타임에서 결정하는 것 
    // Vec<Box<dyn Booky>>: Booky 트레잇을 구현한 다양한 타입을 담을 수 있는 벡터
    let vec_of_booky_things: Vec<Box<dyn Booky>> = vec![
        Box::new(Book {name: "my book".to_string()}), // Book 인스턴스
        Box::new(BigBook),                            // BigBook 인스턴스
        Box::new(my_city)                            // City 인스턴스
    ];
}