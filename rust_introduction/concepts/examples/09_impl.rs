// §9. impl — 메서드와 연관 함수
//
// self의 형태에 따라 동작이 다름:
//   self       → 인스턴스 소유권을 받음 (드물게 사용; 빌더 패턴 등)
//   &self      → 불변 빌림 (읽기만)
//   &mut self  → 가변 빌림 (수정 가능)

struct Counter {
    value: i32,
}

impl Counter {
    // 연관 함수 — self 없음, ::로 호출
    fn new() -> Self {
        Self { value: 0 }
    }

    // 또 다른 연관 함수 — 인자 받아서 만들기
    fn with(start: i32) -> Self {
        Self { value: start }
    }

    // &self — 읽기 전용
    fn read(&self) -> i32 {
        self.value
    }

    // &mut self — 내부 수정
    fn bump(&mut self) {
        self.value += 1;
    }

    // self — 소비형 메서드 (호출 후 인스턴스 사용 불가)
    fn into_value(self) -> i32 {
        self.value
    }
}

fn main() {
    let mut c = Counter::new();             // 연관 함수
    c.bump();                                // &mut self
    c.bump();
    println!("c.read() = {}", c.read());    // &self → 2

    let c2 = Counter::with(100);
    let v = c2.into_value();                 // self를 소비
    // c2.read();   // ❌ c2는 이미 into_value에 의해 소모됨
    println!("v = {}", v);
}
