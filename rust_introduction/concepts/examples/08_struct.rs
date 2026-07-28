// §8. struct 정의

// 일반 구조체
struct Memory {
    slots: std::collections::HashMap<String, f64>,
}

// 튜플 구조체 — 필드 이름 없음
struct Point(f64, f64);

// 단위 구조체 — 데이터 없음 (마커 용도)
struct Marker;

fn main() {
    // 일반 구조체 인스턴스 만들기
    let m = Memory {
        slots: std::collections::HashMap::new(),
    };
    println!("Memory의 슬롯 개수: {}", m.slots.len());

    // 필드 접근은 . 으로
    let p = Point(3.0, 4.0);
    println!("Point: ({}, {})", p.0, p.1);

    // 단위 구조체
    let _marker = Marker;

    // 필드 이름과 변수 이름이 같으면 단축 표기 가능
    let slots = std::collections::HashMap::new();
    let m2 = Memory { slots };  // 동일: Memory { slots: slots }
    println!("m2 슬롯: {}", m2.slots.len());
}
