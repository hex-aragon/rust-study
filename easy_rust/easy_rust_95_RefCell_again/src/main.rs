use std::cell::RefCell;


//runtime checked 

fn main() {
   let my_cell = RefCell::new(String::from("I am a String"));
   //let reference = my_cell.borrow_mut();
   println!("{my_cell:?}");
   *my_cell.borrow_mut() = String::from("I am not a String");
   println!("{my_cell:?}");
   
   let blocking_reference = my_cell.borrow_mut();
   match my_cell.try_borrow_mut() {
     Ok(mut r) => *r = String::from("I am not a String"),
     Err(e) => println!("We got an error: {e}") 
   };

   println!("{my_cell:?}");
}
