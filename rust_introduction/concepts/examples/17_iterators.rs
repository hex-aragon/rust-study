// §17. 반복자와 클로저, map / collect

fn main() {
    // (1) split → iterator → map → collect (mem_calcu6의 Token::split 패턴)
    let text = "1 2 3 4 5";
    let nums: Vec<i32> = text
        .split(char::is_whitespace)         // 함수도 인자로 받음 (fn(char) -> bool)
        .map(|s| s.parse().unwrap())        // 클로저
        .collect();                          // 어디로 모을지는 좌변에서 추론
    println!("{:?}", nums);

    // (2) 함수 이름 그 자체를 map의 인자로 — Token::parse처럼
    let nums2: Vec<i32> = text.split(' ').map(parse_int).collect();
    println!("{:?}", nums2);

    // (3) lazy 평가 — collect/for/sum 같은 소비자가 없으면 실행 안 됨
    let _just_iter = (0..5).map(|x| {
        println!("(map 호출됨 — 출력되면 평가된 것)");
        x * 2
    });
    // 아무것도 출력 안 됨

    let sum: i32 = (0..5).map(|x| x * 2).sum();
    println!("sum = {}", sum);

    // (4) 자주 쓰는 어댑터들
    let evens: Vec<i32> = (1..=10).filter(|n| n % 2 == 0).collect();
    println!("짝수: {:?}", evens);

    let pairs: Vec<(usize, &str)> =
        ["a", "b", "c"].iter().copied().enumerate().collect();
    println!("{:?}", pairs);

    let total: i32 = vec![1, 2, 3, 4].iter().sum();
    println!("총합 {}", total);

    // (5) 클로저는 환경 변수를 캡처할 수 있음
    let bias = 100;
    let with_bias: Vec<i32> = (1..=3).map(|x| x + bias).collect();
    println!("{:?}", with_bias);
}

fn parse_int(s: &str) -> i32 {
    s.parse().unwrap()
}
