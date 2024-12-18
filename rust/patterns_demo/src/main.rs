enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message){
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move{x,y} => println!("Moving to ({}, {})", x, y),
        Message::Write(text) if text.len() < 10 => println!("Short message: {}", text),
        Message::Write(text) => println!("Long message: {}", text),
        Message::ChangeColor(r,g,b)=>println!("Changing color to ({}, {}, {})",r, g, b),
    }
}

fn main() {
    println!("Hello, world!");

    let messages = vec![
        Message::Quit,
        Message::Move {x:10, y:20},
        Message::Write(String::from("Hello")),
        Message::Write(String::from("This is a long message")),
        Message::ChangeColor(255,0,0),
    ];

    for msg in messages { 
        process_message(msg);
    }
}
