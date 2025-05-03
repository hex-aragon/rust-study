//universal logger
struct MyLogger(Vec<&dyn Any>);

// std::any 모듈에서 Any 트레이트와 type_name 함수를 가져옵니다.
// Any 트레이트는 모든 타입의 기본 트레이트입니다.
use std::any::{Any, type_name};

// 테스트용 사용자 정의 타입
struct MyType;

// 입력된 값의 타입에 따라 다른 동작을 수행하는 함수
// &dyn Any는 어떤 타입이든 참조할 수 있는 트레이트 객체입니다.
fn do_stuff_depending(input: &dyn Any) {
    // is::<T>() 메서드를 사용하여 타입을 체크합니다.
    if input.is::<String>() {
        println!("We got a String");
    } else if input.is::<i32>() {
        println!("We have a number");
    } else if input.is::<MyType>() {
        println!("We have a MyType");
    } else {
        println!("Don't know what it is");
    }
}

// 입력된 값을 String으로 다운캐스팅을 시도하는 함수
fn try_to_Get_string(input: &dyn Any) {
    // downcast_ref::<T>()를 사용하여 특정 타입으로 다운캐스팅을 시도합니다.
    // 성공하면 Some(T)를, 실패하면 None을 반환합니다.
    if let Some(a_string) = input.downcast_ref::<String>() {
        println!("We have a String! {a_string}");
    } else {
        println!("We got something else");
    }
}

fn main() {
    // 다양한 타입의 값을 테스트
    do_stuff_depending(&8);  // i32 타입
    do_stuff_depending(&String::from("I am a String"));  // String 타입
    do_stuff_depending(&MyType);  // MyType 타입

    // String으로 다운캐스팅 시도
    try_to_Get_string(&8);  // 실패 (i32는 String이 아님)
    try_to_Get_string(&"Hello there");  // 실패 (&str은 String이 아님)
    try_to_Get_string(&String::from("Hello there i am a string"));  // 성공
}
