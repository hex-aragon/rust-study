// Cow(Clone-on-Write)를 사용하기 위한 import
use std::borrow::Cow;

// User 구조체 정의
// 'a: 수명 매개변수 - 참조의 수명을 명시
#[derive(Debug)]
struct User<'a> {
   // name 필드는 Cow를 사용하여 문자열을 처리
   // &str 또는 String 타입을 모두 처리할 수 있음
   name: Cow<'a, str> 
}

// User 구조체의 구현
impl User<'_> {
    // Cow의 상태를 확인하는 메서드
    fn is_borrowed(&self) {
        match &self.name {
            // Cow::Borrowed: 참조만 가지고 있는 경우
            Cow::Borrowed(name) => println!("It's borrowed : {name}"),
            // Cow::Owned: 소유권을 가지고 있는 경우
            Cow::Owned(name) => println!("It's owned: {name}")
        }
    }
}

fn main() {
    // Cow::Borrowed 생성
    // "User 1"은 문자열 리터럴(&str)이므로 참조만 유지
    let mut user_1 = User {
        name: "User 1".into()  // Cow::Borrowed(&str)로 변환
    };

    // Cow::Owned 생성
    // "User 2".to_string()은 String이므로 소유권을 가짐
    let user_2 = User {
        name: "User 2".to_string().into() // Cow::Owned(String)으로 변환
    };

    // Debug 트레이트를 통해 구조체 출력
    println!("User 1 is {user_1:?} and User 2 is {user_2:?}");
    
    // user_1은 현재 참조 상태
    user_1.is_borrowed(); // "It's borrowed : User 1" 출력
    
    // user_2는 소유권을 가진 상태
    user_2.is_borrowed(); // "It's owned: User 2" 출력

    // to_mut()을 호출하여 가변 참조를 얻음
    // 이 시점에서 Cow::Borrowed에서 Cow::Owned로 변환됨
    user_1.name.to_mut().push('!'); // 문자열 수정
    
    // 이제 user_1은 소유권을 가진 상태로 변경됨
    user_1.is_borrowed(); // "It's owned: User 1!" 출력

    // Cow의 주요 특징:
    // 1. Clone-on-Write: 쓰기가 필요할 때만 복사가 발생
    // 2. 메모리 효율성: 불필요한 복사를 방지
    // 3. 유연성: 참조와 소유권을 상황에 따라 선택
}
