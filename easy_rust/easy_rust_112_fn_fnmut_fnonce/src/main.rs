// 클로저를 매개변수로 받는 세 가지 함수를 정의합니다.
// 각각 Fn, FnMut, FnOnce 트레이트를 구현하는 클로저를 받습니다.

// Fn 트레이트: 클로저가 환경의 값을 소비하지 않고 참조만 하는 경우
fn fn_closure<F>(f: F) where F: Fn(), {
    f();
}

// FnMut 트레이트: 클로저가 환경의 값을 변경할 수 있는 경우
fn fn_mut_closure<F>(mut f: F) where F: FnMut(), {
    f();
}

// FnOnce 트레이트: 클로저가 환경의 값을 소비하는 경우
fn fn_once_closure<F>(f: F) where F: FnOnce(), {
    f();
}

fn main() {
    // 변경 가능한 String 변수를 생성합니다
    let mut my_string = String::from("Hello there");

    // Fn 클로저 예시 (주석 처리됨)
    // fn_closure(|| {
    //   //drop(my_string);  // 컴파일 에러: Fn은 값을 소비할 수 없음
    //   println!("{my_string}");  // 참조만 가능
    // });

    // FnOnce 클로저 예시 (주석 처리됨)
    // fn_once_closure(|| {
    //   drop(my_string);  // 값을 소비하는 것이 가능
    // });

    // FnMut 클로저 예시
    fn_mut_closure(|| {
      my_string.push('a');  // 값을 변경하는 것이 가능
      println!("{my_string}");
     // drop(my_string);  // 값을 소비하는 것은 불가능
    });
}
