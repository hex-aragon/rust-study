// Solution 구조체 정의 - LeetCode 문제를 위한 구조체
struct Solution;

// Solution 구조체에 longest_palindrome 메서드 구현
impl Solution {
   // 가장 긴 팰린드롬 부분문자열을 찾는 공개 함수
   pub fn longest_palindrome(s: String) -> String {
    // 입력 문자열을 문자 단위로 분해하여 벡터로 변환
    let s_chars: Vec<char> = s.chars().collect();
    // 문자열의 길이를 저장
    let n = s_chars.len();
    // 문자열 길이가 2보다 작으면 팰린드롬이 될 수 없으므로 원본 반환
    if n < 2 {
        return s;
    }

    // 가장 긴 팰린드롬의 시작 인덱스를 저장할 변수
    let mut start = 0;
    // 가장 긴 팰린드롬의 길이를 저장할 변수 (최소값 1)
    let mut max_len = 1;

    // 각 위치에서 팰린드롬을 확인
    for i in 0..n {
        // 현재 위치를 중심으로 홀수 길이의 팰린드롬 확인 (예: "aba")
        let len1 = get_palindrome_len(&s_chars, i, i); // 홀수
        // 현재 위치와 다음 위치를 중심으로 짝수 길이의 팰린드롬 확인 (예: "aa")
        let len2 = get_palindrome_len(&s_chars, i, i + 1); // 짝수
        // 홀수와 짝수 중 더 긴 팰린드롬 길이 선택
        let len = len1.max(len2);

        // 더 긴 팰린드롬을 찾으면 정보 업데이트
        if len > max_len {
            max_len = len;
            // 팰린드롬의 시작 인덱스 계산: 중심점에서 (길이-1)/2 만큼 왼쪽
            start = i - (len - 1) / 2;
        }
    }

    // 찾은 팰린드롬을 문자열로 반환
    s[start..start + max_len].to_string()
}
}

// 팰린드롬 길이 계산 함수
// s_chars: 문자 벡터, l: 왼쪽 인덱스, r: 오른쪽 인덱스
fn get_palindrome_len(s_chars: &Vec<char>, mut l: usize, mut r: usize) -> usize {
    // 문자열의 길이
    let n = s_chars.len();
    // l과 r이 범위 내에 있고, 해당 위치의 문자가 같을 때까지 확장
    while l < n && r < n && s_chars[l] == s_chars[r] {
        // l이 0이 되면 더 이상 왼쪽으로 확장할 수 없음
        if l == 0 {
            break;
        }
        // 왼쪽으로 확장
        l -= 1;
        // 오른쪽으로 확장
        r += 1;
    }
    
    // while 문을 빠져나온 후 인덱스 재조정
    // l이 0이고 여전히 팰린드롬이면 l을 그대로, 아니면 l+1
    let final_l = if l == 0 && s_chars[l] == s_chars[r] { l } else { l + 1 };
    // r은 확장이 끝난 후 1을 빼줌
    let final_r = r - 1;

    // 유효한 팰린드롬이면 길이 반환, 아니면 0 반환
    if final_r < final_l {
        0
    } else {
        final_r - final_l + 1
    }
}

// 메인 함수 - 테스트 실행
fn main() {
    // 테스트 케이스 1: "babad" -> "bab" 또는 "aba"
    let test1 = "babad".to_string();
    let result1 = Solution::longest_palindrome(test1.clone());
    println!("테스트 1: 입력 = '{}', 출력 = '{}'", test1, result1);
    
    // 테스트 케이스 2: "cbbd" -> "bb"
    let test2 = "cbbd".to_string();
    let result2 = Solution::longest_palindrome(test2.clone());
    println!("테스트 2: 입력 = '{}', 출력 = '{}'", test2, result2);
    
    // 추가 테스트 케이스들
    let test3 = "racecar".to_string();
    let result3 = Solution::longest_palindrome(test3.clone());
    println!("테스트 3: 입력 = '{}', 출력 = '{}'", test3, result3);
    
    let test4 = "a".to_string();
    let result4 = Solution::longest_palindrome(test4.clone());
    println!("테스트 4: 입력 = '{}', 출력 = '{}'", test4, result4);
    
    let test5 = "aa".to_string();
    let result5 = Solution::longest_palindrome(test5.clone());
    println!("테스트 5: 입력 = '{}', 출력 = '{}'", test5, result5);
}
