// todo!() macro
// type aliases
// alias = different name-
// todo!() - I'll do it later 

// type SkipAndTake = std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>;


// fn skip_five_take_five(input: Vec<char>) -> SkipAndTake {
//     input.into_iter().skip(5).take(5)
// }

// //new type
// type MyString = String;
// type MyOtherString(String);

// impl SomeTrait for MyOtherString {

// }

struct SomeType {
    name: String, 
    number: u8 
}

fn some_function(input: SomeType) -> Vec<SomeType> {
    todo!(); 
    //unimplemented!
}

fn main() {
   // some_function();
   
}
