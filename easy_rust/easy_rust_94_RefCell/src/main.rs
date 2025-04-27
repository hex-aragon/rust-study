//use std::cell::Cell; 

//not thread safe 
//set 
//get 
//small copy types

// runtime checked borrowing rules
// compile time 
// panic 
// unwind the stack 

#[derive(Debug)]
struct User {
    id: u32, 
    year_registered: u32,
    username: String, 
    active: RefCell<bool>
    //Many other fields 
}

use std::cell::RefCell;

fn main() {
    // let my_cell = Cell::new(String::from("I am a String"));
    // my_cell.set(String::from("I am a String??!?!?!?!?"));
    // let my_string = my_cell.get();

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true)
    };
    println!("{user_1:?}");

    let mut first_reference = user_1.active.borrow_mut();
   // let second_reference = user_1.active.borrow_mut();

    println!("{user_1:?}");
    *first_reference = false;

    println!("{user_1:?}");
    drop(first_reference); // drop을 썼을 때 borrow 제거 ? 
    
    println!("{user_1:?}");

}
