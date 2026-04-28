use rand::{Rng, RngExt}; //추가


fn main() {

    

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

    dbg!(ans_input); // => cargo run으로 입력한 값 확인 가능
    if dbg!(ans_input == op1 + op2) {
        println!("정답!");
    } else {
        println!("오답!");
    }


    let op1 = rand::rng().random_range(0..100);
    let op2 = rand::rng().random_range(0..100);
    println!("{} + {} = ??", op1, op2);
    println!("??의 값을 입력하세요:");
    let mut ans_input = String::new(); //사용자의 답을 저장하는 변수
    //표준 입력으로 한 줄을 받아서 ans_input에 대입
    //  std::io::stdin().read_line(&mut ans_input).unwrap();
    //  dbg!(ans_input); // => 키보드로 입력한 값 확인 

    //표준 입력에서 한 줄을 받아서 ans_input 에 대입
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_input을 trim()으로 줄바꿈을 제거하고 parse()로 정수(u32) 타입으로 변환
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input); // => cargo run으로 입력한 값 확인 가능
    if dbg!(ans_input == op1 + op2) {
        println!("정답!");
    } else {
        println!("오답!");
    }

    println!("i32 데이터 범위: {} ~ {}", i32::MIN, i32::MAX);
    println!("u32 데이터 범위: {} ~ {}", u32::MIN, u32::MAX);

}
