/// 두 정수를 더하는 함수
///
/// # 인자
///
/// * `a` - 첫 번째 정수
/// * `b` - 두 번째 정수
///
/// # 반환값
///
/// 두 정수의 합을 반환한다
fn add(a: i32, b: i32) -> i32 {
    // 단순히 두 수를 더해 반환한다
    a + b
}

fn main() {
    // 변수 선언 및 초기화
    let x = 5;
    let y = 7;

    /* 여러 줄 주석 예시
       add 함수를 호출하여 결과를 계산한다 */
    let result = add(x, y);

    // 결과 출력
    println!("{} + {} = {}", x, y, result);


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
            assert_eq!(add(-1, 1), 0);
            assert_eq!(add(0, 0), 0);
        }
    }
}