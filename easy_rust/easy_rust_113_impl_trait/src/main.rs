//impl trait
use std::fmt::Display;

//caller 
fn generic_function<T: Display>(input: T ){
    println!("{input}");
}

//function 
fn impl_function(input: impl Display) {
    println!("{input}");
}

fn returns_a_closure() -> impl Fn(i32) {
    |x| println!("{x}")
}

//type MyString = impl Display; 

//generic 은 caller가 정한다. 
//impl function은 알아서 ?? 
fn main() {
    let my_number = 9;
    let my_closure = returns_a_closure();
    my_closure(my_number);

    generic_function::<u8>(8);
    //impl_function::<u8>(8);
}
