// §7. String vs &str, 문자열 슬라이싱

fn main() {
    // (1) String — 힙에 할당된 가변 문자열, 소유권 보유
    let mut owned: String = String::from("hello");
    owned.push_str(", world");
    println!("owned = {}", owned);

    // (2) &str — 슬라이스, 소유권 없음
    let literal: &str = "I'm a literal";       // 프로그램 바이너리에 박힌 정적 문자열
    let borrow:  &str = &owned;                // String을 &str로 자동 변환
    let part:    &str = &owned[0..5];          // 부분 슬라이스

    println!("{} | {} | {}", literal, borrow, part);

    // (3) 함수 인자는 보통 &str로 받음 — String도 받을 수 있어 유연
    print_twice(literal);
    print_twice(&owned);     // String을 &str로 빌려줌

    // (4) mem_calcu6의 Token::parse 패턴
    let value = "mem_x+";
    if value.starts_with("mem") {
        let mut name = value[3..].to_string();   // "x+" → 새로운 String
        if value.ends_with('+') {
            name.pop();                           // 끝 글자 1개 제거 → "x"
            println!("memory plus: {}", name);
        }
    }

    // (5) 슬라이스의 주의점 — 바이트 인덱스
    let s = String::from("Hi한");      // 'H'=1B, 'i'=1B, '한'=3B (UTF-8)
    println!("'{}'", &s[0..2]);          // OK: "Hi"
    // println!("'{}'", &s[0..3]);       // ❌ 한글 중간을 자름 — 런타임 panic
}

fn print_twice(s: &str) {
    println!("{} {}", s, s);
}
