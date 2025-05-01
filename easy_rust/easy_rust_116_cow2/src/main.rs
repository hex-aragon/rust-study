fn main() {
    // String 생성 방법들 비교

    // 1. From 트레이트 사용: &str에서 String으로 변환
    // String::from()은 From 트레이트의 구현을 사용
    let string_1 = String::from("Hello there");
    
    // 2. Display 트레이트의 to_string() 메서드 사용
    // &str에서 String으로 변환하는 일반적인 방법
    let string_2 = "Hello there".to_string();
    
    // 3. From 트레이트의 into() 메서드 사용
    // 타입 명시가 필요하며, From 트레이트의 구현을 사용
    let string_3: String = "Hello there".into();
    
    // 4. ToOwned 트레이트의 to_owned() 메서드 사용
    // 빌린 데이터에서 소유한 데이터를 생성
    let string_4 = "Hello there".to_owned();

    // Cow 사용 예시
    use std::borrow::Cow;
    
    // Cow::Borrowed: 데이터를 소유하지 않고 참조만 함
    // 메모리 할당 없이 참조만 유지
    let borrowed = Cow::Borrowed("Hello");
    
    // Cow::Owned: 데이터를 소유함
    // 메모리를 할당하여 데이터를 소유
    let owned = Cow::Owned(String::from("World"));
    
    // to_mut()을 호출하면 필요할 때만 복사가 발생
    // 이 예제에서는 문자열이 변경되므로 복사가 발생
    let mut cow = Cow::Borrowed("Hello");
    cow.to_mut().push_str(" World"); // 이 시점에서 복사가 발생
    
    // Cow를 사용한 효율적인 문자열 처리 함수
    // 조건에 따라 참조 또는 소유한 데이터를 반환
    fn process_string(input: &str) -> Cow<str> {
        if input.contains("need_change") {
            // 변경이 필요한 경우에만 복사
            // 대문자로 변환하는 작업이 필요하므로 복사본 생성
            Cow::Owned(input.to_uppercase())
        } else {
            // 변경이 필요 없는 경우 참조만 반환
            // 메모리 할당 없이 참조만 반환
            Cow::Borrowed(input)
        }
    }
    
    // 변경이 필요 없는 경우: 참조만 반환
    let result1 = process_string("Hello");
    
    // 변경이 필요한 경우: 복사본 생성
    let result2 = process_string("need_change");
}
