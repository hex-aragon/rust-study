use std::error::Error;
use std::num::ParseIntError;
use std::fmt::{Formatter, Display};

#[derive(Debug)]
enum CompanyError {
    CouldntConect,
    NotEnoughData,
    UserTimeOut
} 

impl Display for CompanyError {
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "got a CompanyError")
}
}

#[derive(Debug)]
struct BaseError;

impl Display for BaseError {
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
     write!(f, "got a BaseError")
}
}

impl Error for CompanyError {

}

impl Error for BaseError {

}

//fn try_to_make_number(input: &str, float_input: &str) -> Result<i32, ParseIntError> {
// fn try_to_make_number(input: &str, float_input: &str) -> Result<(), ParseIntError> {
//     let my_number = input.parse::<i32>()?;
//     let other_number = input.parse::<f32>()?;
//     Ok(())
// }


fn main() {
    
}
