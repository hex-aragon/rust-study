// anyhow
// thiserror
// anyhow
// serde
// JSON
// {points: "30", age: "8"}

use thiserror::{Error}; // 1.0.69

#[derive(Debug)]
struct User {
    points: u32,
    age:u8
}

impl User {
    fn try_new() -> Result<Self, CompanyError> {
        todo!()
    }
}

#[derive(Error, Debug)]
enum CompanyError {
    #[error("Not enough data")]
    NotEnoughData,
    #[error("Too old: {0} can't be over 120")]
    TooOld(u8),
    #[error("Got {0}, should be under 10,000")]
    TooBig(u32),
    #[error("Must be under 120 and 10,000 points, got {0:?} instead")]
    TooBigAndTooOld(User) //UserBuilder
}

fn main() {

    let some_error = CompanyError::TooBig(20000);
    let second_error = CompanyError::NotEnoughData;
    println!("{some_error}");
    println!("{second_error}");

}