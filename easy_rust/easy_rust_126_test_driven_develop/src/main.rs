// 허용되는 문자들을 정의하는 상수: 숫자(0-9), 더하기(+), 빼기(-), 공백만 허용
const OKAY_CHARACTERS: &str = "1234567890+- ";

// 수학 표현식을 계산하는 함수
// input: &str - 계산할 수학 표현식 문자열
// 반환값: i32 - 계산 결과
fn math(input: &str) -> i32 {
    // 입력 문자열의 모든 문자가 OKAY_CHARACTERS에 포함되어 있는지 검사
    // 허용되지 않은 문자가 있으면 panic 발생
    if !input
        .chars()
        .all(|character| OKAY_CHARACTERS.contains(character))
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
    println!("{input}"); // 처리된 문자열 출력 (디버깅용)

    // 결과를 저장할 벡터: 각 숫자와 연산자를 저장
    let mut result_vec = vec![];
    // 현재 처리 중인 숫자나 연산자를 임시로 저장하는 문자열
    let mut push_string = String::new();

    // 입력 문자열의 각 문자를 순회하며 처리
    for character in input.chars() {
        match character {
            '+' => {
                // + 기호를 만났을 때
                if !push_string.is_empty() {
                    // 현재 push_string이 비어있지 않다면
                    result_vec.push(push_string.clone()); // 현재까지의 문자열을 결과 벡터에 추가
                    push_string.clear(); // push_string을 비워서 다음 숫자를 받을 준비
                }
                // push_string이 비어있으면 아무것도 하지 않음 (연속된 + 기호 무시)
            }
            '-' => {
                if push_string.contains('-') || push_string.is_empty() {
                    // 이미 마이너스가 있거나 빈 문자열인 경우
                    // 예: "-" 또는 "--"와 같은 경우
                    push_string.push(character); // 현재 문자(-)를 push_string에 추가
                } else {
                    // 숫자가 있는 경우 (예: "123-")
                    result_vec.push(push_string.clone()); // 현재까지의 문자열을 결과 벡터에 추가
                    push_string.clear(); // push_string을 비움
                    push_string.push(character); // - 기호를 새로운 push_string의 시작으로 추가
                }
            }
            number => {
                // 숫자를 만났을 때
                if push_string.contains('-') {
                    // push_string에 이미 마이너스가 있는 경우 (예: "-123")
                    result_vec.push(push_string.clone()); // 현재까지의 문자열을 결과 벡터에 추가
                    push_string.clear(); // push_string을 비움
                    push_string.push(number); // 새로운 숫자를 push_string에 추가
                } else {
                    // 마이너스가 없는 경우
                    push_string.push(number); // 현재 숫자를 push_string에 추가
                }
            }
        }
    }

    // 마지막으로 남은 문자열을 벡터에 추가
    result_vec.push(push_string);

    // 계산을 위한 변수 초기화
    let mut total = 0; // 총합을 저장하는 변수
    let mut adds = true; // 현재 연산이 더하기인지 빼기인지 나타내는 플래그
    let mut math_iter = result_vec.into_iter(); // 결과 벡터를 순회하는 이터레이터

    // 이터레이터를 사용하여 각 항목을 처리
    while let Some(entry) = math_iter.next() {
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
        if adds == true {
            total += entry.parse::<i32>().unwrap(); // 더하기
        } else {
            total -= entry.parse::<i32>().unwrap(); // 빼기
        }
    }
    total // 최종 계산 결과 반환
}

// .filter "7 + 9 + 10" -> "7+9+10"
// 7 + 9 + 10+++++++++++++++++++++++-------------


fn main() {
    // 테스트 실행
    let my_number = math("7 + 9 + 10  ++++++++");
}



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
    fn one_minus_two_is_minus_one(){
        assert_eq!(math("1 - 2"), -1);
    }

    // 1 - (-1) = 2 테스트
    #[test]
    fn one_minus_minus_one_is_two(){
        assert_eq!(math("1 - - 1"), 2);
    }

    // 잘못된 문자 입력 시 panic 발생 테스트
    #[test]
    #[should_panic]
    fn panics_when_characters_not_right(){
        math("7 + please add seven");
    }
}
