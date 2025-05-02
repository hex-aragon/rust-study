use std::borrow::Cow;

#[derive(Debug)]
struct User<'a> {
   name: Cow<'a, str> 
}

fn main() {
    // Cow의 주요 장점:
    // 1. 참조를 그대로 사용할 수 있음 (메모리 할당 없음)
    // 2. 필요할 때 소유권을 가져올 수 있음 (to_owned())
    // 3. 필요할 때 가변 참조를 얻을 수 있음 (to_mut())

    // &str 타입의 문자열 리터럴
    let name_1 = "User 1";
    
    // String 타입의 문자열
    let name_2 = "User 2".to_string();

    // Cow::Borrowed 생성
    // name_1은 &str이므로 참조만 유지
    let user_1 = User {
        name: name_1.into()  // Cow::Borrowed(&str)로 변환
    };

    // Cow::Owned 생성
    // name_2는 String이므로 소유권을 가짐
    let user_2 = User {
        name: name_2.into()  // Cow::Owned(String)으로 변환
    };

    // Debug 트레이트를 통해 구조체 출력
    println!("User 1 is {user_1:?} and User 2 is {user_2:?}");

    // Cow의 유연성 예시
    // user_1.name은 &str을 참조하고 있으므로 메모리 할당 없이 사용 가능
    println!("User 1's name length: {}", user_1.name.len());
    
    // user_2.name은 String을 소유하고 있으므로 변경 가능
    let mut user_2 = user_2;
    user_2.name.to_mut().push_str(" (modified)");
    println!("Modified User 2: {user_2:?}");
}
