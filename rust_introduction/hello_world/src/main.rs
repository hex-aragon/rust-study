
fn main() {
    println!("Hello, world!");
    println!();
    println!("Hello, Cargo!");

    let char_a = 'a';
    let str_a = "a";
    let string_b = "b".to_string();
    let b_a = string_b + str_a; 
    println!("{}", b_a);

    let one = 1;
    dbg!(one);

    let two;
    two = 2;
    dbg!(two); 
    
    {
        let three = 3;
        dbg!(three); 
    }
    //dbg!(three);
    
    let three = 1 + 2;
    let six = three * 2;
    dbg!(three);
    dbg!(six);

    let four = 4;
   // four = four * 2; >> error

   let four = 5; //섀도잉
   dbg!(four);

   let mut five = 5; 
   five = five * 2;
   dbg!(five);
}
