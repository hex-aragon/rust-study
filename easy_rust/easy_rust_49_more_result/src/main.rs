use std::num::ParseIntError;

fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn check_if_five(number: i32) -> Result<i32, String>{
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five".to_string())
    }
}

fn parse_number(number: &str) -> Result<i32, ParseIntError> {
    number.parse()
}

fn main() {
    
    // let mut result_vec = Vec::new(); //Vec<Result<i32, String>>
    
    // for number in 2..=7{
    //     result_vec.push(check_if_five(number));
    // }
    
    // println!("{:#?}", result_vec);
    
    // if give_result(6).is_ok() {
    //     println!("It's okay, guys! ");
    // } else {
    //     println!("It's an error, guys!");
    // }
    
    // anyhow - crate >> crate란 라이브러리인데, 사람들이 자주 쓰는게 anyhow
    // 에러관련된 라이브러리다. 
    
    let mut result_vec = vec![];
    result_vec.push(parse_number("8"));
    result_vec.push(parse_number("asdfsadf"));
    result_vec.push(parse_number("8"));
    
    for number in result_vec {
        println!("{:?}", number);
    }
    
}
