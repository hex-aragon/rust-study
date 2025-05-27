// thiserror
// anyhow
// serde
// JSON
// {points: "30", age: "8"}
use thiserror::{Error}; // 1.0.69
use anyhow::{Error as AnyhowError, anyhow}; // 1.0.98

#[derive(Debug)]
struct User {
    points: u32,
    age:u8
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

#[derive(Error, Debug)]
#[error("I couldn't care less")]
struct DontCareError;

// fn do_some_stuff(number: &str, age: u8, points: u32) -> Result<(), DontCareError> {
//     let my_number = number.parse::<i32>().map_err(|e| {
//         println!("Got an error: {e}");
//         DontCareError
//     })?;
//     let my_user = User::try_new(age, points).map_err(|e| {
//         println!("Couldn't make a user: {e}");
//         DontCareError
//     })?;
//    // println!("{my_number}, {my_user:?}");
//     Ok(())

// }

fn do_some_stuff(number: &str, age: u8, points: u32) -> Result<(), AnyhowError> {
    let my_number = number.parse::<i32>().map_err(|_| { 
        anyhow!("Couldn't get a number")
    })?;
    let my_user = User::try_new(age, points).map_err(|_| {
        anyhow!("Couldn't make a user")
    })?;
   // println!("{my_number}, {my_user:?}");
    Ok(())

}

impl User {
    fn try_new(age: u8, points: u32) -> Result<Self, CompanyError> {
        use CompanyError::*;
        match (age, points) {
           
                (age, points) if age > 120 && points > 10000  => {
                    Err(TooBigAndTooOld(User {age, points}))
                },
                (_, p) if p > 10000 => {
                    Err(TooBig(p))
                },
                (a, _) if a > 120 => {
                    Err(TooOld(a))
                },
                _ => Ok(Self {age, points})
            
        }
    }
}


fn main() {

    let some_error = CompanyError::TooBig(20000);
    let second_error = CompanyError::NotEnoughData;
    println!("{some_error}");
    println!("{second_error}");

    let user_requests = vec![
        User::try_new(150, 20000),
        User::try_new(100, 20000),
        User::try_new(200, 1000),
        User::try_new(40, 5000),
        User::try_new(50, 7000)
    ];

    println!("{user_requests:#?}");

    let users = user_requests
        .into_iter()
        .filter_map(|user_request| match user_request {
            Ok(user) => Some(user),
            Err(message) => {
                println!("{message}");
                None 
            }
        })
        .collect::<Vec<User>>();
    
    //println!("{users:?}");
    let try_1 = do_some_stuff("nthoetho", 30, 100);
    let try_2 = do_some_stuff("90", 200, 100);
    let try_3 = do_some_stuff("90", 100, 1998000);
    let try_4 = do_some_stuff("90", 100, 100);

    println!("{try_1:?}, {try_2:?} , {try_3:?} , {try_4:?}");
}