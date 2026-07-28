// §14. Option<T>와 Result<T, E>
//
// Rust에는 null이 없다. 대신 Option/Result로 "값 없음"과 "에러"를 표현.

fn main() {
    // ── Option<T> ──────────────────────────────
    let a: Option<i32> = Some(10);
    let b: Option<i32> = None;

    // unwrap: 안의 값 꺼내기, None이면 panic
    println!("{}", a.unwrap());

    // unwrap_or: 없으면 기본값
    println!("{}", b.unwrap_or(0));

    // unwrap_or_else: 없으면 클로저로 값 생성
    println!("{}", b.unwrap_or_else(|| -1));

    // map: 안의 값에 함수 적용 (없으면 None 그대로)
    let doubled: Option<i32> = a.map(|v| v * 2);
    println!("{:?}", doubled);

    // mem_calcu6 패턴: Option<&V> → copied → unwrap_or
    let map: std::collections::HashMap<&str, f64> = std::collections::HashMap::new();
    let v = map.get("x").copied().unwrap_or(0.0);
    println!("v = {}", v);

    // ── Result<T, E> ───────────────────────────
    let ok: Result<i32, _> = "42".parse();
    let er: Result<i32, _> = "abc".parse();

    println!("{}", ok.unwrap());
    println!("{}", er.unwrap_or(-1));

    // match로 안전하게 분기
    match "3.14".parse::<f64>() {
        Ok(v) => println!("값 {}", v),
        Err(e) => println!("에러 {}", e),
    }

    // ? 연산자 — 에러 자동 전파 (함수가 Result를 반환할 때)
    println!("{:?}", parse_pair("10 20"));
    println!("{:?}", parse_pair("10 abc"));
}

fn parse_pair(s: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    let mut it = s.split_whitespace();
    let a: i32 = it.next().unwrap().parse()?;   // ? = 실패 시 즉시 Err 반환
    let b: i32 = it.next().unwrap().parse()?;
    Ok((a, b))
}
