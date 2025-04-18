
fn main() {
   
    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'Ꮔ';
    let cat_face = '😺';
    println!("first_letter : {}", first_letter);
    println!("space : {}", space);
    println!("other_language_char : {}", other_language_char);
    println!("cat_face : {}", cat_face);
    
    let my_number = 100; 
    //We didn't write a type of integer
    //so Rust chooses i32, Rust always
    //chooses i32 for integers if you don't
    //tell it to use different type 
    
    //println!("{}", my_number as char);
    //>> result is >> error[E0604]: only `u8` can be cast as `char`, not `i32`
  
      
    println!("{}", my_number as u8 as char);
      
    let my_number2 : u8 = 1; 
    println!("{}", my_number2 as char);
    
    let my_number3 : u8 = 2; 
    println!("{}", my_number3 as char);
    
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of string containing 'a': {}", "a".len());
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
    
    
    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!"; //korean for "hi"
    println!("Slice2 is {} bytes.", slice2.len());
    
    println!("Slice is {} bytes and also {} characters", slice.len(), slice.chars().count());
    println!("Slice2 is {} bytes but only {} characters", slice2.len(), slice2.chars().count());
    
  }
  