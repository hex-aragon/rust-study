// §11. enum과 데이터 보유 variant
//
// Rust의 enum은 C와 다르다 — 각 variant가 서로 다른 데이터를 가질 수 있다.
// "이것 또는 저것" 합 타입(sum type).

#[derive(Debug)]
enum Token {
    Number(f64),                  // f64 1개
    Identifier(String),           // String 1개
    Pair(i32, i32),               // 두 개의 i32
    Range { start: i32, end: i32 },   // 이름 있는 필드
    Plus,                          // 데이터 없음
    Minus,
}

fn main() {
    let tokens = vec![
        Token::Number(3.14),
        Token::Identifier(String::from("x")),
        Token::Pair(1, 2),
        Token::Range { start: 0, end: 10 },
        Token::Plus,
        Token::Minus,
    ];

    for t in &tokens {
        // {:?}로 Debug 형식 출력 (derive(Debug) 덕분)
        println!("{:?}", t);

        // variant 별로 데이터 꺼내기
        match t {
            Token::Number(v)            => println!("  → 숫자 {}", v),
            Token::Identifier(name)     => println!("  → 식별자 {}", name),
            Token::Pair(a, b)           => println!("  → 쌍 ({}, {})", a, b),
            Token::Range { start, end } => println!("  → 범위 [{}, {}]", start, end),
            Token::Plus                 => println!("  → +"),
            Token::Minus                => println!("  → -"),
        }
    }
}
