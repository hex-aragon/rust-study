// §18. 튜플 구조분해 (destructuring)

fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)        // 마지막 식이 반환값
}

fn main() {
    // (1) 튜플 만들기 / 인덱스 접근
    let pair: (i32, &str) = (10, "ten");
    println!("{} {}", pair.0, pair.1);

    // (2) let 구조분해
    let (q, r) = divmod(17, 5);
    println!("17 / 5 = {} ... {}", q, r);

    // (3) 함수 반환을 한 번에 분해
    let (sum, product) = compute(3, 4);
    println!("sum={}, product={}", sum, product);

    // (4) destructuring assignment (Rust 1.59+)
    //     기존 변수에 한꺼번에 다시 대입. 둘 다 mut여야 함.
    let mut a = 0;
    let mut b = 0;
    (a, b) = (1, 2);
    println!("a={}, b={}", a, b);

    // mem_calcu6에서 자주 쓰는 패턴:
    let mut index = 0_usize;
    let mut result;
    (result, index) = mul_step(index);
    println!("result={}, index={}", result, index);
    (result, index) = mul_step(index);
    println!("result={}, index={}", result, index);

    // (5) 일부만 받고 나머지는 무시
    let triple = (1, 2, 3);
    let (first, _, last) = triple;
    println!("{} {}", first, last);
}

fn compute(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn mul_step(i: usize) -> (f64, usize) {
    (i as f64 * 1.5, i + 1)
}
