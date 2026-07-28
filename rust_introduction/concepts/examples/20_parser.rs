// §20. 재귀 하강 파서 — mem_calcu6의 핵심 알고리즘 축약 버전
//
// 문법:
//   expr        = additive
//   additive    = multiplicative (('+'|'-') multiplicative)*
//   multiplicative = primary (('*'|'/') primary)*
//   primary     = number | '(' additive ')'
//
// 각 함수는 (계산값, 다음 토큰 인덱스)를 반환한다.
// 우선순위는 함수 호출 깊이로 자연스럽게 표현됨 — primary가 가장 안쪽이라 먼저 묶임.

#[derive(Debug, PartialEq)]
enum Tok {
    Num(f64),
    Plus, Minus, Star, Slash,
    LParen, RParen,
}

fn tokenize(s: &str) -> Vec<Tok> {
    s.split_whitespace().map(|t| match t {
        "+" => Tok::Plus,
        "-" => Tok::Minus,
        "*" => Tok::Star,
        "/" => Tok::Slash,
        "(" => Tok::LParen,
        ")" => Tok::RParen,
        _   => Tok::Num(t.parse().unwrap()),
    }).collect()
}

fn additive(tokens: &[Tok], i: usize) -> (f64, usize) {
    let (mut acc, mut i) = multiplicative(tokens, i);
    while i < tokens.len() {
        match &tokens[i] {
            Tok::Plus  => { let (v, n) = multiplicative(tokens, i + 1); acc += v; i = n; }
            Tok::Minus => { let (v, n) = multiplicative(tokens, i + 1); acc -= v; i = n; }
            _ => break,
        }
    }
    (acc, i)
}

fn multiplicative(tokens: &[Tok], i: usize) -> (f64, usize) {
    let (mut acc, mut i) = primary(tokens, i);
    while i < tokens.len() {
        match &tokens[i] {
            Tok::Star  => { let (v, n) = primary(tokens, i + 1); acc *= v; i = n; }
            Tok::Slash => { let (v, n) = primary(tokens, i + 1); acc /= v; i = n; }
            _ => break,
        }
    }
    (acc, i)
}

fn primary(tokens: &[Tok], i: usize) -> (f64, usize) {
    match &tokens[i] {
        Tok::Num(v) => (*v, i + 1),
        Tok::LParen => {
            // 괄호 안은 다시 additive로 — 재귀
            let (v, n) = additive(tokens, i + 1);
            assert_eq!(tokens[n], Tok::RParen);
            (v, n + 1)            // ')' 다음 위치
        }
        _ => unreachable!(),     // ← 매크로! 괄호 () 아님
    }
}

fn calc(src: &str) -> f64 {
    let toks = tokenize(src);
    let (val, end) = additive(&toks, 0);
    assert_eq!(end, toks.len(), "전부 다 소비되지 않음");
    val
}

fn main() {
    let cases = [
        "1 + 2 * 3",          // 7
        "( 1 + 2 ) * 3",      // 9
        "10 - 2 - 1",         // 7  (왼쪽 결합)
        "2 * ( 3 + 4 ) - 1",  // 13
        "100 / 4 / 5",        // 5  (왼쪽 결합)
    ];

    for src in cases {
        println!("{:>20} = {}", src, calc(src));
    }
}
