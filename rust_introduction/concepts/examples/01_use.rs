// §1. use 선언과 모듈 경로
//
// 표준 라이브러리(std)에서 여러 항목을 한 번에 가져오는 방법.
// {} 안에 중첩이 가능하므로 같은 모듈 하위 항목들을 묶을 수 있다.

use std::{
    collections::{hash_map::Entry, HashMap},
    io::stdin,  // 이 예제에선 안 쓰지만 mem_calcu6과 같은 형태를 보여주기 위해 포함
};

fn main() {
    // HashMap을 짧은 이름으로 바로 사용 가능
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("a"), 1);

    // hash_map::Entry도 use 했으므로 Entry::Occupied / Entry::Vacant 형태로 매칭 가능
    match map.entry(String::from("a")) {
        Entry::Occupied(e) => println!("이미 있음: {}", e.get()),
        Entry::Vacant(_) => println!("아직 없음"),
    }

    // stdin은 import만 해두고 사용 안 함 — Rust는 사용하지 않는 import에 경고를 준다
    let _ = stdin;
}
