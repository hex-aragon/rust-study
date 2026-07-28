// §15. 표준 입력
//
// Playground에서도 STDIN 입력 가능 (실행 패널 위에 "Stdin" 박스).

use std::io::stdin;

fn main() {
    println!("이름을 한 줄씩 입력 (빈 줄로 종료):");

    // stdin().lines()는 Iterator<Item = io::Result<String>> — 한 줄씩 줌.
    // 줄바꿈(\n)은 자동 제거됨.
    for line in stdin().lines() {
        let line = line.unwrap();   // io 에러 시 panic
        if line.is_empty() {
            break;
        }
        println!("→ 안녕, {}!", line);
    }

    println!("종료");
}
