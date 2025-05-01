//hint to the compiler
// #[warn(dead_code)]
// #[warn(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_variables)] //!가 있어야 전체 공간에 해당됨
//#[cfg()] // configuration
#[cfg(target_os = "linux")]
fn do_something() {
    println!("I am running in Linux");
}

#[cfg(target_os = "windows")]
fn do_something() {
    println!("I am running in Windows");
}

struct JustAStruct {}

fn main() {
    let _some_char = 'a';
    do_something();
}
