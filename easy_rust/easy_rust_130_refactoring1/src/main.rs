// Calculator 구조체 정의
// Clone 트레이트를 구현하여 복사 가능하도록 함
#[derive(Clone)]
struct Calculator {
    results: Vec<String>,      // 계산 결과를 저장하는 벡터
    current_input: String,     // 현재 처리 중인 입력을 저장하는 문자열
    total: i32,               // 총 계산 결과
    adds: bool                // 현재 연산이 더하기인지 빼기인지 나타내는 플래그
}

// Calculator 구조체의 구현
impl Calculator {
    // 새로운 Calculator 인스턴스를 생성하는 함수
    fn new() -> Self {
        Self {
            results: vec![],           // 빈 결과 벡터로 초기화
            current_input: String::new(), // 빈 입력 문자열로 초기화
            total: 0,                  // 총합을 0으로 초기화
            adds: true                 // 기본 연산을 더하기로 설정
        }
    }
}

// 허용되는 문자들을 정의하는 상수: 숫자(0-9), 더하기(+), 빼기(-), 공백만 허용
const OKAY_CHARACTERS: &str = "1234567890+- ";

// 수학 표현식을 계산하는 함수
// input: &str - 계산할 수학 표현식 문자열
// 반환값: i32 - 계산 결과
fn math(input: &str) -> i32 {
    // 입력 검증:
    // 1. 모든 문자가 OKAY_CHARACTERS에 포함되어 있는지 검사
    // 2. 처음 두 문자 중 하나 이상이 숫자인지 검사
    if !input
        .chars()
        .all(|character| OKAY_CHARACTERS.contains(character)) ||
        !input.chars().take(2).any(|character| character.is_numeric())
    {
        panic!("Please only input numbers, +-, or spaces");
    }

    // 입력 문자열 전처리 과정
    // 1. trim_end_matches: 문자열 끝에서 +, -, 공백을 모두 제거
    // 2. filter: 문자열 내의 모든 공백을 제거
    // 3. collect: 처리된 문자들을 다시 문자열로 변환
    let input = input
        .trim_end_matches(|x| "+- ".contains(x))
        .chars()
        .filter(|character| *character != ' ')
        .collect::<String>();
    
    // 새로운 Calculator 인스턴스 생성
    let mut calculator = Calculator::new();

    // 입력 문자열의 각 문자를 순회하며 처리
    for character in input.chars() {
        match character {
            '+' => {
                // + 기호를 만났을 때
                if !calculator.current_input.is_empty() {
                    // 현재 입력이 비어있지 않다면
                    calculator.results.push(calculator.current_input.clone()); // 현재 입력을 결과 벡터에 추가
                    calculator.current_input.clear(); // 입력을 비워서 다음 숫자를 받을 준비
                }
                // 입력이 비어있으면 아무것도 하지 않음 (연속된 + 기호 무시)
            }
            '-' => {
                if calculator.current_input.contains('-') || calculator.current_input.is_empty() {
                    // 이미 마이너스가 있거나 빈 문자열인 경우
                    // 예: "-" 또는 "--"와 같은 경우
                    calculator.current_input.push(character); // 현재 문자(-)를 입력에 추가
                } else {
                    // 숫자가 있는 경우 (예: "123-")
                    calculator.results.push(calculator.current_input.clone()); // 현재 입력을 결과 벡터에 추가
                    calculator.current_input.clear(); // 입력을 비움
                    calculator.current_input.push(character); // - 기호를 새로운 입력의 시작으로 추가
                }
            }
            number => {
                // 숫자를 만났을 때
                if calculator.current_input.contains('-') {
                    // 입력에 이미 마이너스가 있는 경우 (예: "-123")
                    calculator.results.push(calculator.current_input.clone()); // 현재 입력을 결과 벡터에 추가
                    calculator.current_input.clear(); // 입력을 비움
                    calculator.current_input.push(number); // 새로운 숫자를 입력에 추가
                } else {
                    // 마이너스가 없는 경우
                    calculator.current_input.push(number); // 현재 숫자를 입력에 추가
                }
            }
        }
    }

    // 마지막으로 남은 입력을 결과 벡터에 추가
    calculator.results.push(calculator.current_input);

    // 계산을 위한 변수 초기화
    let mut total = 0; // 총합을 저장하는 변수
    let mut adds = true; // 현재 연산이 더하기인지 빼기인지 나타내는 플래그
    let math_iter = calculator.results.into_iter(); // 결과 벡터를 순회하는 이터레이터

    // 이터레이터를 사용하여 각 항목을 처리
    for entry in math_iter {
        if entry.contains('-') {
            // 마이너스 기호가 포함된 항목 처리
            if entry.chars().count() % 2 == 1 {
                // 마이너스의 개수가 홀수인 경우 (예: "-", "---")
                adds = match adds {
                    true => false, // 현재 더하기면 빼기로 변경
                    false => true, // 현재 빼기면 더하기로 변경
                };
                continue;
            } else {
                // 마이너스의 개수가 짝수인 경우 (예: "--", "----")
                continue; // 연산자 부호는 그대로 유지
            }
        }
        // 숫자 처리
        if adds {
            total += entry.parse::<i32>().unwrap(); // 더하기
        } else {
            total -= entry.parse::<i32>().unwrap(); // 빼기
        }
    }
    total // 최종 계산 결과 반환
}

fn main() {
    // Calculator 인스턴스 생성 및 테스트 실행
    let mut calculator = Calculator::new();
    math("1 + 1");
}

// 테스트 모듈
#[cfg(test)]
mod tests {
    use super::math; // 상위 모듈의 math 함수 사용

    // 1 + 1 = 2 테스트
    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }

    // 1 - 2 = -1 테스트
    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!(math("1 - 2"), -1);
    }

    // 1 - (-1) = 2 테스트
    #[test]
    fn one_minus_minus_one_is_two() {
        assert_eq!(math("1 - - 1"), 2);
    }

    // 잘못된 문자 입력 시 panic 발생 테스트
    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        math("7 + please add seven");
    }
}
