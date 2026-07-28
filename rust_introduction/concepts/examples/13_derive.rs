// §13. #[derive(...)] — 트레이트 자동 구현
//
// 일정 조건만 맞으면 컴파일러가 트레이트(인터페이스)를 자동 구현해준다.

#[derive(Debug, PartialEq, Clone)]
struct Point { x: i32, y: i32 }

#[derive(Debug, PartialEq)]
enum Color { Red, Green, Blue }

fn main() {
    let p = Point { x: 1, y: 2 };

    // Debug → {:?} 가능
    println!("{:?}", p);                  // Point { x: 1, y: 2 }
    println!("{:#?}", p);                 // 보기 좋게 들여쓴 형태

    // Clone → .clone() 가능
    let q = p.clone();
    println!("{:?}", q);

    // PartialEq → ==, != 가능
    println!("p == q: {}", p == q);       // true
    let r = Point { x: 0, y: 0 };
    println!("p == r: {}", p == r);       // false

    // enum도 동일하게 적용
    let c1 = Color::Red;
    let c2 = Color::Red;
    println!("{:?} == {:?} → {}", c1, c2, c1 == c2);

    // assert_eq!는 PartialEq + Debug가 둘 다 필요 (값 비교 + 실패 시 출력)
    assert_eq!(c1, Color::Red);
    println!("assert 통과");
}
