
use std::{    
    collections::{hash_map::Entry, HashMap},
    io::stdin,
};

struct Memory {
   slots: HashMap<String, f64>,
}

impl Memory {
    fn new() -> Self {
        Self {
           slots: HashMap::new(),
        }
    }

    fn add(&mut self, slot_name: String, prev_result: f64) -> f64 {
        
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += prev_result;
                *entry.get()
            }
            Entry::Vacant(entry) => {
                entry.insert(prev_result);
                prev_result
            }
        }
    }

    fn get(&self, slot_name: &str) -> f64{
        self.slots.get(slot_name).copied().unwrap_or(0.0)
    }
    
}

// [개념] 프로그램의 진입점. 표준 입력을 한 줄씩 읽어 계산하는 루프.
fn main() {

    // [개념] let mut — 가변 바인딩. 이후 메서드 호출로 내부 상태가 바뀌어야 하므로 mut 필요.
    let mut memory = Memory::new();
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {

        //한 줄씩 읽고 빈 줄이면 종료
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        //토큰 나열로 분할
        let tokens = Token::split(&line);

        //수식 평가
        match &tokens[0] {
            Token::MemoryPlus(memory_name) => {
                //메모리에 더하기
                let memory_name = memory_name.to_string();
                let result = memory.add(memory_name, prev_result);
                print_output(result);
            }
            Token::MemoryMinus(memory_name) => {
                //메모리에 빼기
                let memory_name = memory_name.to_string();
                let result = memory.add(memory_name, -prev_result);
                print_output(result);
            }
            _ => {
                //수식 계산
                // let left = eval_token(&tokens[0], &memory);
                // let right = eval_token(&tokens[0], &memory);
                //책은 아래 right라고 함 memory 자리에 있는 부분
                let result = eval_expression(&tokens , &memory);

                //결과 표시
                print_output(result);
                prev_result = result;
            }
        }


    }
}


#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    MemoryRef(String),
    MemoryPlus(String),
    MemoryMinus(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen
}

impl Token {
    fn parse(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash,
            _ if value.starts_with("mem") => {
                let mut memory_name = value[3..].to_string();
                if value.ends_with('+'){
                    memory_name.pop(); //끝에서 글자 하나를 삭제
                    Self::MemoryPlus(memory_name)
                } else if value.ends_with('-') {
                    memory_name.pop();//끝에서 글자 하나를 삭제
                    Self::MemoryMinus(memory_name)
                } else {
                    Self::MemoryRef(memory_name)
                }
            }
            "(" => Self::LParen,
            ")" => Self::RParen,
            _ => Self::Number(value.parse().unwrap()),
    }
    }

    fn split(text: &str) -> Vec<Self>{
        text.split(char::is_whitespace).map(Self::parse).collect()
    }
}

fn eval_token(token: &Token, memory: &Memory) -> f64 {

   

    match token {
        Token::Number(value) => {
            //숫자이므로 그 값을 반환
            *value
        }
        Token::MemoryRef(memory_name) => {
            //메모리 참조이므로 메모리 값 반환
            memory.get(memory_name)
        }
        _ => {
            //입력이 올바르지 않으면 여기로
            unreachable!()
        }
    }
}

fn eval_expression(tokens: &[Token], memory: &Memory) -> f64{
    let (result, index) = eval_additive_expression(tokens, 0, memory);
    //올바르게 계산 했다면 index는 식의 끝을 가리킴
    assert_eq!(tokens.len(), index);
    result
    //eval_additive_expression(tokens, memory)
}

fn eval_additive_expression(
    tokens: &[Token],
    index: usize,
    memory: &Memory
) -> (f64 , usize) {
    let mut index = 0;
    let mut result; 
    (result, index) = eval_multiplicative_expression(
        tokens, index, memory
    );

    while index < tokens.len() {
        match &tokens[index] {
            Token::Plus => {
                let (value, next) = eval_multiplicative_expression(
                tokens,
                index + 1,
                memory
            );
            result += value;
            index = next;
            }
            Token::Minus => {
                let (value, next) = eval_multiplicative_expression(
                    tokens,
                    index + 1,
                    memory
                );
                result -= value;
                index = next;
            }
            _ => break,
        }
    }
    (result, index)
}

fn eval_multiplicative_expression(
    tokens: &[Token],
    index: usize,
    memory: &Memory
) -> (f64, usize) {
    let mut index = index;
    //let mut result = eval_token(&tokens[index], memory);
    let mut result;
    (result, index) = eval_primary_expression(tokens, index, memory);
    //index += 1;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Asterisk => {
                // result *= eval_token(&tokens[index + 1], memory);
                // index += 2;
                let (value, next) = eval_primary_expression(
                    tokens,
                    index + 1,
                    memory
                );
                result *= value;
                index = next; 
            }
            Token::Slash => {
                // result /= eval_token(&tokens[index + 1], memory);
                // index += 2;
                let (value, next) = eval_primary_expression(
                    tokens,
                    index + 1,
                    memory
                );
                result /= value;
                index = next; 
            }
            _ => break,
        }
    }
    (result, index)
}

fn eval_primary_expression(
    tokens: &[Token],
    index: usize,
    memory: &Memory
) -> (f64, usize) {
    let first_token = &tokens[index];
    match first_token {
        Token::LParen => {
            //여는 괄호로 시작하므로 괄호의 다음 토큰부터 식 계산 시작 
            let (result, next) = eval_additive_expression(
                tokens,
                index + 1,
                memory
            );
            //tokens[next]는 닫는 괄호이어야 함
            assert_eq!(Token::RParen, tokens[next]);
            //닫는 괄호까지 처리했으므로ㅠ 다음 토큰 위치를 반환
            (result, next + 1)
        }
        Token::Number(value) => {
            //숫자이므로 그 값과 다음 위치를 반환
            (*value, index + 1)
        }
        Token::MemoryRef(memory_name) => {
            //메모리 참조이므로 메모리 값과 다음 위치를 반환
            (memory.get(memory_name), index + 1)
        }
        _ => {
            //입력이 올바르지 않으면 여기로
            unreachable();
        }
    }
}

// [개념] 자유 함수(어떤 타입에도 속하지 않는 일반 함수). 결과 출력 전용.
fn print_output(value: f64){
    // [개념] println! — 매크로(컴파일 타임 포맷 검사). {}는 Display 트레이트 사용.
    println!(" => {}", value);
}
