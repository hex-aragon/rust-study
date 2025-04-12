enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar 
}

enum Number {
    U32(u32), //tuple ? , enum은 데이터를 가질 수 있다. 
    I32(i32)
    
}

fn get_number(input: i32) -> Number {
    // let number = match input.is_positive() {
    //     true => Number::U32(input as u32),
    //     false => Number::I32(input)
    // };
    // number 

    match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input)
    }
    
}

fn main() {
  use Star::*;
  let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
  
  for star in starvec {
      match star as u32 {
          size if size <= 80 => println!("Not the biggest star: {}", size),
          size if size >= 80 => println!("Pretty big star: {}", size),
          _ => println!("Some other star")
      }
  }
  
  println!("What about DeadStar? It is : {}", DeadStar as u32);
  
    let my_vec = vec![get_number(-800), get_number(8)]; //Vec<Number>
    
    
    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {} ", number),
            Number::I32(number) => println!("It's an i32 with the value {}", number)
        }
    }
}
