// §12. 패턴 매칭 — match, 가드, 바인딩

fn main() {
    // (1) 값 매칭 — 모든 경우를 다 다뤄야 한다 (exhaustiveness)
    let n = 3;
    match n {
        1 => println!("하나"),
        2 | 3 => println!("둘 또는 셋"),         // | 로 OR
        4..=10 => println!("4부터 10 사이"),      // 범위
        _ => println!("그 외"),                   // 와일드카드
    }

    // (2) enum + 데이터 바인딩
    #[derive(Debug)]
    enum Msg {
        Hello(String),
        Move { x: i32, y: i32 },
        Quit,
    }

    let msgs = [
        Msg::Hello(String::from("rust")),
        Msg::Move { x: 10, y: 20 },
        Msg::Quit,
    ];

    for m in &msgs {
        match m {
            Msg::Hello(name) => println!("hi {}", name),
            Msg::Move { x, y } => println!("({}, {})", x, y),
            Msg::Quit => println!("bye"),
        }
    }

    // (3) 가드 (guard) — 추가 조건을 if로 붙임
    let value = "mem_x+";
    let kind = match value {
        "+" => "plus",
        "-" => "minus",
        v if v.starts_with("mem") => "memory",   // 가드 ★
        _ => "number",
    };
    println!("{} → {}", value, kind);

    // (4) if let — 한 가지 패턴만 신경 쓸 때
    let some = Some(42);
    if let Some(x) = some {
        println!("값 = {}", x);
    }

    // (5) while let — 패턴 매칭 루프
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("pop: {}", top);
    }
}
