// 'static 특별한 lifetime 
// 'static은 프로그램 전체에서 살아있는 라이프타임을 의미합니다.

struct Book<'a> { // Book 구조체, 'a 라이프타임 파라미터를 가짐
    name : &'a  str,         // 'a 라이프타임을 가진 문자열 참조
    second_name: &'a str     // 'a 라이프타임을 가진 또 다른 문자열 참조
}

struct Adventurer<'a> { // Adventurer 구조체, 'a 라이프타임 파라미터를 가짐
    name: &'a str,      // 'a 라이프타임을 가진 문자열 참조
    hit_points: u32     // 체력(정수)
}

// implicit == not said 
// elided == not shown 
// 어떤 라이프타임인지 쓸지 정했기 때문에 _ 씀 
// 러스트에서 <'_> 이런 표현 많음 
// 위 주석은 라이프타임 표기법에 대한 설명입니다.
// 'elided'는 생략된, 'implicit'는 명시하지 않은 의미입니다.

impl Adventurer<'_> { // Adventurer의 라이프타임을 명시적으로 쓰지 않고 '_로 생략
    fn take_damage(&mut self) { // self를 가변 참조로 받아 체력을 깎음
        self.hit_points -= 20; // 체력 20 감소
        println!("{} has {} hit points left!", self.name, self.hit_points); // 남은 체력 출력
    }
}

fn main() { 
    let my_book_title = "my_book_title".to_string(); // String 타입의 책 제목 생성
    // 현재 main 함수에서는 my_book_title만 생성하고, 구조체 인스턴스는 만들지 않음
}
