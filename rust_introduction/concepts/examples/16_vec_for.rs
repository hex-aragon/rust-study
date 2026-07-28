// §16. Vec<T>, 슬라이스 &[T], for 루프

fn main() {
    // (1) Vec<T>: 가변 동적 배열, 힙 할당
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?} len={}", v, v.len());

    // (2) vec! 매크로로 한 줄에 만들기
    let v2 = vec![10, 20, 30, 40];

    // (3) 인덱싱 — 범위 벗어나면 panic
    println!("{}", v2[0]);
    println!("{:?}", &v2[1..3]);   // 슬라이스 &[i32]

    // (4) 슬라이스 인자로 받으면 Vec와 배열 모두 받을 수 있다
    print_sum(&v);
    print_sum(&v2);
    print_sum(&[7, 8, 9]);

    // (5) for 루프의 세 형태
    let v3 = vec![String::from("a"), String::from("b")];

    for s in &v3 {            // 빌림 (&String) — v3는 그대로 사용 가능
        println!("- {}", s);
    }

    for s in &v3 {            // 다시 빌림
        println!("길이 {}", s.len());
    }

    for s in v3 {             // 소유권 이동 — v3는 이후 사용 불가
        println!("이동: {}", s);
    }
    // println!("{:?}", v3); // ❌ moved
}

fn print_sum(items: &[i32]) {
    let mut total = 0;
    for x in items {        // x: &i32
        total += x;          // i32는 + 연산자가 &i32와 i32 모두 지원
    }
    println!("합 = {}", total);
}
