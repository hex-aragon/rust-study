use core::num;

use rand::RngExt; //추가

fn main() {
    let mut num_of_correct = 0;
    while num_of_correct < 3 {
        //quiz_mode를 무작위로 1 또는 2로 설정
        let quiz_mode = rand::rng().random_range(1..=2);
        match quiz_mode {
            1 => loop {
                //quiz_mode가 1이면 덧셈 퀴즈
                let op1 = rand::rng().random_range(0..100);
                let op2 = rand::rng().random_range(0..100);
                println!("{} + {} = ??", op1, op2);
                println!("??의 값을 입력하세요:");
                let mut ans_input = String::new(); //사용자의 답을 저장하는 변수
                //표준 입력으로 한 줄을 받아서 ans_input에 대입
                //std::io::stdin().read_line(&mut ans_input).unwrap();
                //dbg!(ans_input); // => 키보드로 입력한 값 확인

                //표준 입력에서 한 줄을 받아서 ans_input 에 대입
                std::io::stdin().read_line(&mut ans_input).unwrap();

                // ans_input을 trim()으로 줄바꿈을 제거하고 parse()로 정수(u32) 타입으로 변환
                let ans_input = ans_input.trim().parse::<i32>().unwrap();

                //dbg!(ans_input); // => cargo run으로 입력한 값 확인 가능
                if  ans_input == op1 + op2 {
                    println!("정답!");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("오답!");
                }
            }
            2 => {
                //quiz_mode가 2이면 뺄셈 퀴즈
                let op1 = rand::rng().random_range(0..100);
                let op2 = rand::rng().random_range(0..100);
                println!("{} - {} = ??", op1, op2);
                println!("??의 값을 입력하세요:");
                let mut ans_input = String::new(); //사용자의 답을 저장하는 변수
                //표준 입력으로 한 줄을 받아서 ans_input에 대입
                //  std::io::stdin().read_line(&mut ans_input).unwrap();
                //  dbg!(ans_input); // => 키보드로 입력한 값 확인

                //표준 입력에서 한 줄을 받아서 ans_input 에 대입
                std::io::stdin().read_line(&mut ans_input).unwrap();

                // ans_input을 trim()으로 줄바꿈을 제거하고 parse()로 정수(u32) 타입으로 변환
                let ans_input = ans_input.trim().parse::<i32>().unwrap();

               // dbg!(ans_input); // => cargo run으로 입력한 값 확인 가능
                if ans_input == op1 - op2 {
                    println!("정답!");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("오답!");
                }
            }
            _ => unreachable!(),
        }
    }


    googoodan();
    googoodan2();
}

fn googoodan() {
    let mut flag = false;
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == 56 {
                flag = true;
                println!("googoodan");
                //56이 포함되는 것을 확인했으므로 바깥쪽 반복문까지 한 번에 탈출하고 시ㅍ음
                break;
            }
        }
    }
}

fn googoodan2() {
    let mut flag = false;
    'outer: for i in 1..=9 {
        for j in 1..=9 {
            if i * j == 56 {
                flag = true;
                //'outer 바깥으로 탈출
                println!("googoodan2");
                break 'outer;
            }
        }
    }
}
