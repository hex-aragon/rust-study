// §3. 기본 타입 (i32, usize, f64, bool, char) + parse

fn main() {
    // 정수
    let a: i32 = -5;
    let b: u32 = 10;
    let i: usize = 100;        // 인덱스/길이용 — 기계어에서 포인터 폭과 동일
    println!("i32={}, u32={}, usize={}", a, b, i);

    // 부동소수점 — 기본은 f64
    let pi: f64 = 3.141592;
    let half: f32 = 0.5_f32;   // 접미사로 타입 강제
    println!("f64={}, f32={}", pi, half);

    // 정수와 실수 사이는 명시적 변환만 가능 (as)
    let n: i32 = 42;
    let f: f64 = n as f64;
    let m: i32 = pi as i32;    // 절삭 (3)
    println!("{} {} {}", n, f, m);

    // bool, char
    let t: bool = 3 > 1;
    let ch: char = '한';        // 4바이트 유니코드 1글자
    println!("{} {}", t, ch);

    // 문자열에서 숫자로: parse는 Result 반환 → unwrap
    let parsed: f64 = "3.14".parse().unwrap();
    println!("parsed = {}", parsed);

    // turbofish ::<>로 타입을 직접 지정할 수도 있음
    let parsed2 = "42".parse::<i32>().unwrap();
    println!("parsed2 = {}", parsed2);
}
