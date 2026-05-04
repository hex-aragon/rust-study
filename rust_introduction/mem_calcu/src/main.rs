use std::io::stdin;


// fn do_nothing(){

// }

// fn say_hello(){
//     println!("Hello Rust");
// }

// fn add_value(left: f64, right: f64) -> f64 {
//     left + right 
// }

fn main() {

    
    // do_nothing();

    // say_hello();

    // print_value(123.0);

    // let result = add_value(1.0, 2.0) + 3.0;

    // print_value(result);

    let mut memory: f64 = 0.0;
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
        if tokens[0] == "mem+" {
            memory += prev_result;
            print_output(memory);
            continue;
        } else if tokens[0] == "mem-" {
            memory -= prev_result; 
            print_output(memory);
            continue;
        }

        //수식 계산
        let left: f64 = tokens[0].parse().unwrap();
        let right: f64 = tokens[2].parse().unwrap();

        let left = if tokens[0] == "mem" {
            memory
        } else {
            tokens[0].parse().unwrap()
        };

        let right = if tokens[2] == "mem"{
            memory 
        } else {
            tokens[2].parse().unwrap()
        };

        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            // "+" => add_values(left, right),
            // "-" => subtract_values(left, right),
            // "*" => multiply_values(left, right),
            // "/" => divide_values(left, right),
            _ => {
                //입력이 올바르지 않으면 여기로
                unreachable!();
            }
        };

        //결과 표시
        // println!(" => {}", result);
        print_output(result);
        prev_result = result;
    }
}



fn print_output(value: f64){
    println!(" => {}", value);
}

// fn add_values(left: f64, right: f64) -> f64{
//     left + right
// }

// fn subtract_values(left: f64, right: f64) -> f64{
//     left - right
// }

// fn multiply_values(left: f64, right: f64) -> f64{
//     left * right
// }

// fn divide_values(left: f64, right: f64) -> f64{
//     left / right
// }