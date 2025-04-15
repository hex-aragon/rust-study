use std::fmt::Display;
use std::cmp::PartialOrd;

//fn compare_and_print<T: Display, U: Display + PartialOrd>(statement: T, num_1:U, num_2: U) {
// fn compare_and_print<T, U>(statement: T, num_1:U, num_2: U) 
// where 
//     T: Display,
//     U: Display + PartialOrd,

fn compare_and_print<T, U>(statement: T, num_1: U, num_2: U) 
where 
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{}! Is {} greater than {} ? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

fn main() {
    compare_and_print("Listen up!", 9 , 8 );
}
