fn main() {
    println!("Hello, world!");

    //조건문

    let number = 6;
    let result = if number % 2 == 0 {"짝수"} else {"홀수"};
    println!("숫자 {}는 {}입니다.", number, result);

    //반복문
    for i in 1..=5{
        println!("{}의 제곱: {}", i, i * i);
    }

    //이터레이터
    let numbers = vec![1,2,3,4,5];
    let sum: i32 = numbers.iter().sum();
    println!("합계: {}", sum);

    //패턴 매칭
    let message  = Message::Write(String::from("안녕하세요"));
    match message {
        Message::Quit => {println!("종료")}
        Message::Move {x, y} => {println!("이동: x={}, y={}", x, y)}
        Message::Write(text) => {println!("텍스트: {}", text)}
        Message::ChangeColor(r,g,b) => {println!("색상 변경: R={}, G={}, B={}",r ,g, b)}
    }

    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_if_expression(){
            let number = 6; 
            let result = if number % 2 == 0 {"짝수"} else {"홀수"};
            assert_eq!(result, "짝수");
        }

        #[test]
        fn test_iterator(){
            let numbers = vec![1,2,3,4,5];
            let sum: i32 = numbers.iter().sum();
            assert_eq!(sum, 15);
        }

        #[test]
        fn test_pattern_matching() {

            enum Message {
                Quit,
                Move {x: i32, y: i32},
                Write(String),
                ChangeColor(i32, i32, i32),
            }
            
            let message = Message::Write(String::from("테스트"));
            match message {
                Message::Write(text) => assert_eq!(text, "테스트"),
                _ => panic!("예상치 못한 메시지 유형"),
            }
        }

    }
}
