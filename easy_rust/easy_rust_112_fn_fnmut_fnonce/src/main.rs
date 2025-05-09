// Display 트레이트를 사용하기 위한 import
use std::fmt::Display;

// 제네릭 함수: 호출자가 타입을 명시적으로 지정해야 함
// <T: Display>는 T가 Display 트레이트를 구현해야 함을 의미
fn generic_function<T: Display>(input: T) {
    println!("{input}");
}

// impl trait를 사용한 함수: 컴파일러가 타입을 추론
// 호출자가 타입을 명시할 필요가 없음
fn impl_function(input: impl Display) {
    println!("{input}");
}

// 클로저를 반환하는 함수
// impl Fn(i32)는 i32를 매개변수로 받는 클로저를 반환함을 의미
fn returns_a_closure() -> impl Fn(i32) {
    |x| println!("{x}")
}

// 주석 처리된 코드: impl trait는 타입 별칭으로 사용할 수 없음
// type MyString = impl Display;

// C 언어와의 호환성을 위한 구조체 정의
// #[repr(C)]는 C 언어의 메모리 레이아웃과 동일하게 구조체를 정렬
#[repr(C)]
struct SomeRustStruct {
    one: u8,
    two: u16
}

// 테스트 함수 정의
// #[test]는 이 함수가 테스트 함수임을 나타냄
// #[should_panic]는 이 테스트가 패닉을 발생시켜야 성공함을 의미
#[test]
#[should_panic]
fn tests_a_thing() {
   assert_eq!(8, 9);  // 8과 9가 다르므로 패닉 발생
}

// 외부에서 호출 가능한 함수 정의
// #[no_mangle]은 함수 이름을 변경하지 않도록 지정
// C/C++ 등 다른 언어에서 호출할 수 있게 함
#[no_mangle]
fn some_function() {
    // 함수 내용
}

// 일반 함수 정의
fn some_function_1() {
    // 함수 내용
}

// 주석 처리된 테스트 함수
// #[deprecated]는 이 함수가 더 이상 사용되지 않음을 나타냄
// since: 버전 정보
// note: 대체 함수에 대한 설명
#[deprecated(since="0.1", note = "Please use the other function now")]
fn tests_another_thing(){
    // 함수 내용
}

fn main() {
    // 클로저 사용 예시
    let my_number = 9;
    let my_closure = returns_a_closure();
    my_closure(my_number);

    // 제네릭 함수 호출: 타입을 명시적으로 지정
    generic_function::<u8>(8);
    
    // impl trait 함수 호출: 타입을 명시할 수 없음 (컴파일 에러)
    // impl_function::<u8>(8);

    // 더 이상 사용되지 않는 함수 호출 (컴파일러 경고 발생)
    tests_another_thing();
}
