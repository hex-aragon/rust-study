//rgb

fn match_colours(rbg: (u32, u32, u32) ) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Every colour has at least 10")
    }
}

fn match_number(input: i32) {
    //number 사용 ? , @ and 조건 
    //number => println!("It's the number", number ),
    match input {
    number @  0..=10 => println!("It's between 0 and 10, It's the number {}", number),
        _ => println!("It's greater than ten")
    }
}

fn main() {
    
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    
    
    match_colours(first);
    match_colours(second);
    match_colours(third);
    
    let my_number = 10;
    // let some_variable = match my_number {
    //     10 => 8,
    //     _  => "Not ten"
    // }; >> error integer
    
    //let some_variable = if my_number == 10 {8} else {"Somethin else"};
    let some_variable = if my_number == 10 {8} else {15};
    println!("{}", some_variable);
    
    match_number(10);
    match_number(15);
    
    
}