// §6. 라이프타임 (Lifetime)
//
// 라이프타임 = 참조가 유효한 기간.
// 컴파일러는 댕글링 참조(이미 해제된 메모리)를 쓰는 코드를 거부한다.

fn main() {
    // (1) 댕글링 참조 시도 — 컴파일 거부
    // let r;
    // {
    //     let x = 5;
    //     r = &x;     // ❌ x는 이 블록이 끝나면 사라짐
    // }
    // println!("{}", r);

    // (2) 보통은 라이프타임을 적지 않아도 컴파일러가 추론 (elision rules)
    let s = String::from("hello world");
    println!("{}", first_word(&s));

    // (3) 입력 참조 여러 개 → 어느 것을 반환할지 모르면 명시 필요
    let a = String::from("apple");
    let b = String::from("banana");
    println!("긴 쪽: {}", longer(&a, &b));

    // (4) 구조체에 참조를 보관하려면 라이프타임 필수
    let title = String::from("Rust 입문");
    let book = Book { title: &title };
    println!("책: {}", book.title);
}

// 라이프타임 생략 (elision) — 사실 아래와 동일하다:
//   fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap()
}

// 두 입력 모두 같은 'a로 묶고, 반환도 'a로 표시
// 의미: a, b, 반환값이 모두 같은 범위 안에서 살아있어야 한다.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// 구조체에 참조를 담을 때는 'a 명시
struct Book<'a> {
    title: &'a str,
}
