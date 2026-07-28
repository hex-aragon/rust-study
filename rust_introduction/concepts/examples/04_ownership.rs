// §4. 소유권 (Ownership)
//
// 규칙 3가지:
//   1) 모든 값은 소유자(owner)가 정확히 한 명.
//   2) 소유자가 스코프를 벗어나면 값은 자동 해제(drop).
//   3) 값 대입/인자 전달 = 소유권 이동(move). 원본은 더 이상 못 씀.
//      단, Copy 트레이트가 구현된 타입은 복사된다 (i32, f64, bool, char, usize, ...).

fn main() {
    // (1) String은 힙에 저장 → move 발생
    let s1 = String::from("hello");
    let s2 = s1;                 // 소유권이 s1 → s2로 이동
    // println!("{}", s1);       // ❌ borrow of moved value: `s1`
    println!("s2 = {}", s2);

    // (2) Copy 타입은 복사
    let n1: i32 = 5;
    let n2 = n1;                 // i32는 Copy → n1도 계속 사용 가능
    println!("n1={}, n2={}", n1, n2);

    // (3) 함수 인자도 똑같이 적용
    let owned = String::from("world");
    take_string(owned);
    // println!("{}", owned);    // ❌ owned는 take_string 안에서 drop됨

    let copied = 42_i32;
    take_i32(copied);
    println!("copied 여전히 사용 가능: {}", copied);

    // (4) 반환으로 소유권 돌려받기
    let s = give_back(String::from("ping"));
    println!("돌려받음: {}", s);

    // (5) 빌려주면 move가 아니므로 그대로 사용 가능 (자세한 건 §5 borrow 참조)
    let s = String::from("rust");
    print_len(&s);
    println!("여전히 사용 가능: {}", s);
}

fn take_string(s: String) {
    println!("받음: {}", s);
}   // ← 여기서 s가 drop됨

fn take_i32(n: i32) {
    println!("i32 복사본 받음: {}", n);
}

fn give_back(s: String) -> String {
    s   // 마지막 식이 반환값 (세미콜론 없음에 주의)
}

fn print_len(s: &String) {
    println!("길이: {}", s.len());
}
