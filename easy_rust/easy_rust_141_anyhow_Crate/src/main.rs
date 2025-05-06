//this
use anyhow::{anyhow, Error, Context}; // anyhow 크레이트에서 필요한 타입과 트레잇 가져오기
//use std::error::Error; // 표준 라이브러리의 Error 트레잇(주석 처리됨)

// Box<dyn Error> 대신 anyhow::Error를 사용하는 함수
//fn try_to_make_numbers(int: &str, float: &str) -> Result<(), Box<dyn Error>> {
fn try_to_make_numbers(int: &str, float: &str) -> Result<(), Error> {
    // 문자열을 i32로 파싱, 실패 시 컨텍스트 추가
    let my_integer = int.parse::<i32>().with_context(|| "Extra info is here")?; 
    // 문자열을 f64로 파싱, 실패 시 컨텍스트 추가
    let my_float = float.parse::<f64>().with_context(|| "Extra float info is here")?; 
    
    // 에러를 직접 생성하는 예시(주석 처리됨)
    // let x = 9;
    // if x == 9 {
    //     return Err(anyhow!("Uh oh, x shouldn't be 9"));
    // }
    
    Ok(()) // 성공 시 빈 튜플 반환
}

fn main() {
    // "8"은 유효한 i32, "tnohentho"는 유효하지 않은 f64
    let first_try = try_to_make_numbers("8","tnohentho");
    
    // "tnohentho"는 유효하지 않은 i32, "8.7"은 유효한 f64
    let second_try = try_to_make_numbers("tnohentho","8.7");
    
    // 결과 출력 (디버그 포맷)
    println!("{first_try:?}");  // Err(...)
    println!("{second_try:?}"); // Err(...)
}