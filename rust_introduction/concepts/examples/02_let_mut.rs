// §2. let / let mut / shadowing
//
// 변수 바인딩의 세 가지 모드를 비교.

fn main() {
    // (1) let — 불변 바인딩
    let x = 10;
    println!("x = {}", x);
    // x = 20;  // ❌ 컴파일 에러: cannot assign twice to immutable variable

    // (2) let mut — 가변 바인딩
    let mut y = 10;
    println!("y(전) = {}", y);
    y = 20;
    println!("y(후) = {}", y);

    // (3) shadowing — 같은 이름으로 새 let, 타입까지 바뀔 수 있음
    let s = "42";              // s: &str
    let s: i32 = s.parse().unwrap();   // s: i32 (이전 s를 가림)
    let s = s + 1;             // s: i32, 또 다른 새 변수
    println!("s = {}", s);     // 43

    // mem_calcu6에서 자주 보이는 shadowing 패턴
    let line = String::from(" hello\n");
    let line = line.trim();    // String을 trim한 &str로 가림
    println!("[{}]", line);

    // for 루프 안에서 같은 이름 재사용
    for line in ["a", "b", "c"] {
        let line = line.to_string();   // &str을 String으로 가림
        println!("- {}", line);
    }
}
