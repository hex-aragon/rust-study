// AsRef 
use std::fmt::Display;


/fn print_it<T: Display + AsRef<str>>(input: T) {
//fn print_it<T: Display>(input: T) {
    println!("{input}");
}

fn main() {

    print_it("Please print me");
    print_it(String::from("Please print me22"));
    //print_it(9);

}
