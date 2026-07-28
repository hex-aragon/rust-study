// §5. 빌림 (Borrowing)
//
// 규칙: 어떤 시점에든
//   - 불변 참조(&T) 여러 개   OR
//   - 가변 참조(&mut T) 정확히 하나
// 둘은 공존 못함 → 데이터 경합(race) 컴파일 타임 차단.

fn main() {
    // (1) 불변 빌림은 여러 개 OK
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} | {} | {}", s, r1, r2);

    // (2) 가변 빌림은 단 하나
    let mut t = String::from("hi");
    let m = &mut t;
    m.push_str(", world");
    println!("{}", m);
    // let m2 = &mut t;     // ❌ 동시에 두 개의 가변 빌림은 금지
    // println!("{} {}", m, m2);

    // (3) 빌림 끝난 뒤엔 다시 빌릴 수 있음 (NLL: Non-Lexical Lifetimes)
    let mut x = 0;
    let r = &x;
    println!("{}", r);   // 여기서 r의 사용이 끝남
    let rm = &mut x;     // OK — 위의 r은 더 이상 활성 아님
    *rm += 1;
    println!("{}", x);

    // (4) mem_calcu6의 패턴 — 메서드의 self
    let mut counter = Counter { value: 0 };
    counter.bump();              // &mut self로 빌림
    counter.bump();
    println!("값 = {}", counter.read());   // &self로 빌림
}

struct Counter { value: i32 }

impl Counter {
    fn bump(&mut self) {           // 가변 빌림 — 내부 수정
        self.value += 1;
    }
    fn read(&self) -> i32 {        // 불변 빌림 — 읽기만
        self.value
    }
}
