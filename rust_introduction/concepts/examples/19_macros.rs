// §19. 매크로 (! 기호)
//
// 매크로 = 컴파일 시점에 코드를 펼치는 코드 생성기. 함수와 달리 가변 인자, 타입에 따른 동작 가능.

fn main() {
    // (1) println! — 형식 문자열
    println!("Hello");
    println!("값: {}", 42);                       // {} → Display
    println!("디버그: {:?}", vec![1, 2, 3]);       // {:?} → Debug
    println!("예쁘게: {:#?}", vec![1, 2, 3]);
    println!("이름 인자: {x} + {y} = {z}", x = 1, y = 2, z = 3);
    println!("정렬: [{:>10}]", "right");          // 오른쪽 정렬, 폭 10
    println!("소수점: {:.2}", 3.14159);            // 소수점 2자리
    println!("16진수: {:x}", 255);                 // ff

    // (2) format! — 출력 대신 String을 만듦
    let s: String = format!("({}, {})", 1, 2);
    println!("{}", s);

    // (3) eprintln! — 표준 에러로 출력
    eprintln!("이건 stderr");

    // (4) vec! — Vec<T>를 짧게 만듦
    let v = vec![1, 2, 3];
    let zeros = vec![0; 5];                      // 0이 5개
    println!("{:?} {:?}", v, zeros);

    // (5) assert / assert_eq / assert_ne
    assert!(1 + 1 == 2, "산수 망함");
    assert_eq!("hi".len(), 2);
    assert_ne!(1, 2);

    // (6) unreachable!() — "여긴 절대 안 와야 한다"는 표시
    //                     도달하면 panic. 매크로니까 ! 필수!
    let x = 1;
    let label = match x {
        0 => "zero",
        1 => "one",
        _ => unreachable!("x가 0 또는 1만 들어올 줄 알았는데 {}", x),
    };
    println!("{}", label);

    // (7) todo!() — 미구현. 컴파일은 되지만 실행 시 panic.
    // todo!("나중에 구현");

    // (8) panic!() — 즉시 종료
    // panic!("심각한 에러: {}", "예시");

    // (9) dbg!() — 표현식과 그 값을 stderr로 출력 + 값 그대로 반환
    let y = dbg!(3 * 7);
    println!("y = {}", y);
}
