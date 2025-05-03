//downcasting -> dynamically making concrete 
//&dyn Any

struct MyType;

use std::any::{Any, type_name};

//fn get_type_name<T: Any>(input: T) { unused type = input 
fn get_type_names<T: Any, U: Any>(_: T, _: U) {
    let type_of_t  = type_name::<T>();
    let type_of_u  = type_name::<U>();
    println!("First type : {type_of_t}");
    println!("Second type : {type_of_u}");
}


fn main() {
    get_type_names(8, "string");
    get_type_names(vec![8], 7);
    get_type_names(MyType, 6);
    get_type_names(10, true);
    get_type_names(vec!['a'], MyType);
}