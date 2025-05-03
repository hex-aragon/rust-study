#![feature(panic_internals)]
use std::panic::{set_hook, take_hook};

//const IMPORTANT_INFO : Vec<i32> = vec![8,9,10];

fn main() {
    // 중요한 코드의 상태를 나타내는 변수
    let mut important_code = 400;

    // panic hook을 설정
    // panic이 발생했을 때 실행되는 사용자 정의 동작을 지정할 수 있음
    set_hook(Box::new(|panic_info| {
        // payload를 강제로 설정하려고 시도 (실제로는 효과 없음, 예시 코드)
        panic_info.set_payload(&9);
        // let x = 9;
        // println!("don't forget about x: {x}");
        println!("Didn't get a 200 code yet"); // 200 코드가 아직 나오지 않았음을 알림
        // panic의 payload(메시지 등)를 출력
        println!("Panic info: {:?}", panic_info.payload().downcast_ref::<&str>());
    }));

    // 아래 panic! 매크로는 주석 처리되어 있어 실제로 panic이 발생하지 않음
    // panic!("Oh the humanity");
    // let important_number = 9;
    // panic!("Oh the humanity!");

    // 정상적으로 파싱되는 문자열 -> i32로 변환 (에러 없음)
    let my_number = "1234".parse::<i32>().unwrap();
    // 중요한 코드가 성공적으로 실행되었음을 나타내는 값으로 변경
    important_code = 200;
    // panic hook을 해제 (기존 hook을 가져옴)
    let _ = take_hook();
    // 에러가 발생하는 파싱 (panic 발생, 하지만 위에서 hook을 해제했으므로 사용자 정의 hook은 동작하지 않음)
    let other_number = "nthoent89782".parse::<i32>().unwrap();
} 
