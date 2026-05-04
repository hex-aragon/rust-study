use std::io::stdin;

struct Memory {
    slots: Vec<String, f64>,
}

impl Memory {
    fn new(slots: Vec<f64>) -> Self {
        Memory { slots }
    }
}

fn main() {

    let mut memory = Memory {
        slots: vec![],
    }
    let mut prev_result: f64 = 0.0; 

    for line in stdin().lines() {

        //한 줄씩 읽고 빈 줄이면 종료
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        //공백 문자로 구분
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        //메모리에 기록 
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with('+') {
            add_and_print_memory(&mut memory, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            add_and_print_memory(&mut memory, tokens[0], - prev_result);
            continue;
        }


        let left: f64 = eval_token(tokens[0], &memory);
        let right: f64 = eval_token(tokens[2], &memory);
        let result = eval_expression(left, tokens[1], right);
        print_output(result);
        prev_result = result;
    }
}

fn print_output(value: f64){
    println!(" => {}", value);
}



fn add_and_print_memory(memories: &mut Memory, token: &str, prev_result: f64) {

  let slot_name = &token[3..token.len() - 1];
  //모든 메모리 탐색
  for slot in memory.slots.iter_mut() {
    if slot.0 == slot_name{
        //메모리를 찾았으므로 값을 변경하고 표시 
        slot.1 += prev_result;
        print_output(slot.1);
        return;
    }
  }
  //메모리를 찾지 못하면 마지막 요소에 추가
  memory.slots.push(slot_name.to_string(), prev_result);
  print_output(prev_result);


//   let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
//   memories.slots[slot_index] += prev_result;
//   print_output(memories.slots[slot_index]);
}

fn eval_token(token: &str, memory: &Memory) -> f64 {
  if token.starts_with("mem") {
    let slot_name = &token[3..];
    //모든 메모리 탐색 
    for slot in &memory.slots {
      if slot.0 == slot_name {
        //메모리를 찾았으므로 값을 반환
        return slot.1;
      }
    }
    0.0 // 또는 적절한 기본값
  } else {
    token.parse().unwrap()
  }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
         "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                //입력이 올바르지 않으면 여기로
                unreachable!();
            }
    }
}