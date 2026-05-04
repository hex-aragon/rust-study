// use std::io::stdin;

// [개념] use 선언 — 다른 모듈의 항목을 현재 스코프로 끌어와 짧은 이름으로 사용.
// [개념] 중괄호 `{}` 그룹화 — std 아래의 여러 경로를 한 번에 묶어 가져올 수 있음.
use std::{
    // [개념] HashMap = 키-값 저장소(해시 테이블). Entry는 "키가 있든 없든" 한 번의 조회로
    //        분기 처리할 수 있게 해주는 열거형(enum).
    collections::{hash_map::Entry, HashMap},
    // [개념] stdin() = 표준 입력 핸들을 반환하는 함수.
    io::stdin,
} // [버그] use 문 끝에는 세미콜론(;)이 필요. 현재는 누락되어 컴파일 에러 발생.

// [개념] struct = 여러 필드를 묶은 사용자 정의 타입. 여기서는 "메모리 슬롯"들을 보관.
struct Memory {
   // slots: Vec<String, f64>,                      // [참고] Vec은 타입 인자가 1개라 이 형태는 잘못됨.
   // [개념] HashMap<String, f64> — 키는 소유 문자열(String), 값은 64비트 실수(f64).
   slots: HashMap<String, f64>,
}

// [개념] impl 블록 = 특정 타입에 메서드/연관함수를 정의하는 곳.
impl Memory {
    // [개념] 연관 함수(static과 비슷). self를 받지 않으므로 `Memory::new()`처럼 호출.
    // [개념] 반환 타입 `Self`는 "이 impl이 달린 타입(=Memory)"의 별칭.
    fn new() -> Self {
        // [개념] 구조체 리터럴. 필드를 채워 새 인스턴스 생성.
        Self {
           // slots: vec![]                          // [참고] vec![]는 Vec용 매크로라 HashMap에 못 씀.
           // [개념] HashMap::new()는 빈 해시맵을 만든다. 용량은 점차 늘어남.
           slots: HashMap::new(),
        }
    }

    // [개념] &mut self = 인스턴스를 "가변 참조"로 빌림(필드 변경 가능).
    // [개념] &str = 문자열 슬라이스(소유권 없는 문자열 뷰).
    fn add_and_print(&mut self, token: &str, prev_result: f64){
        // [개념] 슬라이싱: token[3..token.len() - 1]
        //        - 인덱스 3부터 끝-1까지의 부분 문자열(바이트 인덱스 기준).
        //        - 예: "mem0+" → 0..len-1 = "0" 추출.
        // [개념] .to_string() = &str을 소유 String으로 복사(HashMap의 키로 넣기 위함).
        let slot_name = token[3..token.len() - 1].to_string();

        // [개념] entry API — 키가 있으면 Occupied, 없으면 Vacant 분기를 한 번에 처리.
        //        get + insert를 따로 부를 때 발생하는 이중 조회와 차용 충돌을 피함.
        match self.slots.entry(slot_name) {
            // [개념] Occupied = 키가 이미 존재. `mut entry`로 값 변경 가능하게 받음.
            Entry::Occupied(mut entry) => {
                //메모리를 찾았으면 값을 변경하고 표시
                // [개념] entry.get_mut() = 내부 값에 대한 &mut f64. `*`로 역참조해 += 적용.
                *entry.get_mut() += prev_result;
                // [개념] entry.get()은 &f64. `*`로 역참조해 f64 값을 함수에 전달(f64는 Copy 타입).
                print_output(*entry.get());
            }
            // [개념] Vacant = 키가 없음. insert로 새 항목을 끼워 넣고 그 자리의 &mut을 돌려받음.
            Entry::Vacant(entry) => {
                //메모리 찾지 못하면 요소를 추가
                entry.insert(prev_result);
                print_output(prev_result);
            }
        }
    }

    // [개념] &self = 불변 참조로 빌림(읽기만 함). 동시에 여러 곳에서 빌릴 수 있음.
    fn eval_token(&self, token: &str) -> f64{
        // [개념] starts_with("mem") = 문자열 접두사 검사 → bool 반환.
        if token.starts_with("mem") {
            // [개념] &token[3..] = "mem" 뒤 부분만 빌리는 슬라이스. 새 String을 만들지 않음.
            let slot_name = &token[3..];
           // self.slots.get(slot_name) 반환값은 Option<&f64>
           // Option 내용물이 참조라면 값을 돌려줄 수 없으므로
           // copied() 메서드로 Option<f64> 타입으로 변경
            //메모리를 찾지 못하면 값으로 0.0 사용
            // [개념] HashMap::get → Option<&V>. 키가 없을 수도 있음을 타입으로 표현.
            // [개념] .copied() — Option<&f64> → Option<f64> (f64는 Copy 타입이라 가능).
            // [개념] .unwrap_or(0.0) — Some이면 그 값, None이면 기본값 0.0 반환.
            self.slots.get(slot_name).copied().unwrap_or(0.0)
            //또는 다음과 같이 구현
            // match self.slots.get(slot_name){
            // //메모리를 찾았으므로 값을 반환
            // Some(value) => *value,
            // 메모리를 찾지 못하면 초깃값 반환
            // None => 0.0,
            // }
            //
        } else {
            // [개념] str::parse() — 문자열을 다른 타입으로 변환. 반환은 Result<T, E>.
            // [개념] .unwrap() — 성공이면 값, 실패면 패닉(학습용; 실서비스는 보통 ? 사용).
            // [개념] 반환 타입이 f64이므로 추론기가 parse::<f64>로 결정함.
            token.parse().unwrap()
        }
    }
}

// [개념] 프로그램의 진입점. 표준 입력을 한 줄씩 읽어 계산하는 루프.
fn main() {

    // [개념] let mut — 가변 바인딩. 이후 메서드 호출로 내부 상태가 바뀌어야 하므로 mut 필요.
    let mut memory = Memory {
        slots: vec![],                      // [버그] slots는 HashMap인데 vec![]를 넣어 타입 불일치.
                                            //        Memory::new() 또는 HashMap::new()로 고쳐야 함.
    }                                       // [버그] 문 끝 세미콜론(;) 누락.
    // [개념] 직전 계산 결과를 저장하는 변수. 초깃값 0.0(f64).
    let mut prev_result: f64 = 0.0;

    // [개념] stdin().lines() — 표준 입력을 한 줄씩 yield하는 이터레이터(Result<String, _> 항목).
    // [개념] for ... in 패턴 — 이터레이터를 자동으로 next() 호출하며 순회.
    for line in stdin().lines() {

        //한 줄씩 읽고 빈 줄이면 종료
        // [개념] Result::unwrap — Ok면 값, Err면 패닉. I/O 오류는 학습용으로 단순 처리.
        // [개념] 외부 변수 line(Result)를 같은 이름의 line(String)으로 섀도잉.
        let line = line.unwrap();
        if line.is_empty() {
            // [개념] break — 가장 가까운 루프를 빠져나감.
            break;
        }

        //공백 문자로 구분
        // [개념] split — 구분자로 잘라 부분 문자열 이터레이터를 만든다(슬라이스 &str을 yield).
        // [개념] char::is_whitespace는 함수 포인터로 전달되어 "공백 종류 모두" 처리.
        // [개념] collect::<Vec<&str>>() — 이터레이터를 컬렉션으로 모음. 좌변 타입으로 추론.
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        //메모리에 기록
        // [개념] 첫 토큰이 "mem"으로 시작하는지 한 번만 검사해 두 분기에서 재사용.
        let is_memory = tokens[0].starts_with("mem");
        // [개념] && — 단축평가 논리 AND. 앞이 false면 뒤를 계산하지 않음.
        // [개념] ends_with('+') — char 인자도 받음(&str/&char/&[char]/closure 모두 지원).
        if is_memory && tokens[0].ends_with('+') {
            //add_and_print_memory(&mut memory, tokens[0], prev_result);

            // [개념] continue — 루프의 다음 반복으로 즉시 진행.
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            // [개념] 부호를 뒤집어 동일 함수 재사용(- prev_result는 단항 마이너스).
            add_and_print_memory(&mut memory, tokens[0], - prev_result);   // [버그] 이 함수는 현재 정의돼 있지 않음.
            continue;
        }


        // [개념] 좌/우 피연산자를 토큰에서 평가. eval_token은 자유 함수로 호출됨.
        // [버그] 이 호출들은 자유 함수 시그니처(eval_token(token, &memory))를 가정하지만,
        //        위에는 Memory 메서드(self.eval_token)만 정의돼 있어 매칭되지 않음.
        let left: f64 = eval_token(tokens[0], &memory);
        let right: f64 = eval_token(tokens[2], &memory);
        // [개념] 연산자 토큰(+, -, *, /)에 따라 결과를 계산.
        let result = eval_expression(left, tokens[1], right);
        print_output(result);
        // [개념] 다음 라인의 mem± 연산을 위해 마지막 결과 저장.
        prev_result = result;
    }
}

// [개념] 자유 함수(어떤 타입에도 속하지 않는 일반 함수). 결과 출력 전용.
fn print_output(value: f64){
    // [개념] println! — 매크로(컴파일 타임 포맷 검사). {}는 Display 트레이트 사용.
    println!(" => {}", value);
}



// fn add_and_print_memory(memories: &mut Memory, token: &str, prev_result: f64) {

//   let slot_name = &token[3..token.len() - 1];
//   //모든 메모리 탐색
//   for slot in memory.slots.iter_mut() {        // [참고] Vec일 때의 옛 구현. HashMap에선 entry API가 더 간결.
//     if slot.0 == slot_name{                    // [참고] 튜플의 .0 / .1 — Vec<(String, f64)>이었을 때 사용.
//         //메모리를 찾았으므로 값을 변경하고 표시
//         slot.1 += prev_result;
//         print_output(slot.1);
//         return;                                // [개념] return — 함수에서 즉시 빠져나감.
//     }
//   }
//   //메모리를 찾지 못하면 마지막 요소에 추가
//   memory.slots.push(slot_name.to_string(), prev_result);   // [참고] push는 Vec용. HashMap은 insert.
//   print_output(prev_result);

// }

// fn eval_token(token: &str, memory: &Memory) -> f64 {
//   if token.starts_with("mem") {
//     let slot_name = &token[3..];
//     //모든 메모리 탐색
//     for slot in &memory.slots {                // [참고] HashMap을 &로 순회하면 (&K, &V) 튜플이 yield됨.
//       if slot.0 == slot_name {
//         //메모리를 찾았으므로 값을 반환
//         return slot.1;
//       }
//     }
//     0.0 // 또는 적절한 기본값                   // [개념] 마지막 식이 세미콜론 없이 오면 함수 반환값.
//   } else {
//     token.parse().unwrap()
//   }
// }

// [개념] 좌/우 피연산자와 연산자 문자열을 받아 결과를 계산.
fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    // [개념] match — 패턴 매칭. 모든 경우를 다루지 않으면 컴파일 에러(망라성 검사).
    match operator {
         // [개념] 각 갈래는 식(expression). 세미콜론 없이 값 반환.
         "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            // [개념] _ = 와일드카드 패턴. 위에서 안 잡힌 모든 값.
            _ => {
                //입력이 올바르지 않으면 여기로
                // [개념] unreachable!() — "여기는 절대 실행 안 됨"을 표현하는 매크로. 도달하면 패닉.
                unreachable!();
            }
    }
}
