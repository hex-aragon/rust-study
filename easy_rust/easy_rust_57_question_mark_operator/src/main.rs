// The question mark operator 
// ? 

use std::num::ParseIntError;

let parsed_number = input.parse::<u16>()?.to_string().parse::<u32>()?.to_string().parse::<i32>()?; 

//.await? 
//add a ? each time to check 


fn parse_str(input: &str) ->  Result<(), ParseIntError> { 
//fn parse_str(input: &str) ->  Result<i32, ParseIntError> { 
    //question mark operator 쓸 때 무조건 리절트로 결과가 반환되어야 함 
    let parsed_number = input.parse::<i32>()?; //return error 
   // Ok(parsed_number) //; 붙이면 () 리턴함, 에러 발생함 
    //parsed_number.adsfas(); >> 타입 체크 
    println!("It worked! {}", parsed_number);
    Ok(())
}

fn main() {
   //Ok() Err()
   
   for item in vec!["Seven", "8", "9.0", "nice", "6060"] {
    //   let parsed = parse_str(item);
    //   println!("{:?}", parsed);
    parse_str(item);
       
   }
   
}