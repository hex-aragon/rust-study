struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, 
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //구조체와 이름을 같에 선언 
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn increment_width(&mut self, delta: u32){
        self.width += delta;
    }
}

struct AlwaysEqual;


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true, 
    }
}

struct Color(i32, i32, i32);


struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
	fn new(width: u32, height: u32) -> Rectangle {
		Rectangle { width, height }
	}
}


enum Fruits {
    Apple,
    Banana,
    Strawberry
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// #[derive(Debug)]
// enum Option<T> {
//     None, 
//     Some(T),
// }
#[derive(Debug)]
enum Result<T, E> {
	Ok(T),
	Err(E),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    
    println!("username: {}, email: {}, sign_in_count: {}, active : {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    
    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1
    };
    println!("user2.email: {} user2.username = {}", user2.email, user2.username );
    
    let color = Color(100, 152, 54);
    println!("{}, {}, {}", color.0, color.1, color.2);
    
    // let _ae = AlwaysEqual;
    // println!("{}", _ae);
    
    //쉐도잉 써서 mut 넣고 가변 구조체로 변경 
     let mut user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    
    user1.active = false;
    println!("active: {}", user1.active);
    
    let mut rect = Rectangle {width: 30, height: 50}; 
    println!("면적: {}", rect.area());
    
    rect.increment_width(32);
    println!("{}", rect.area());
    
    let rect2 = Rectangle2::new(30,50);
    println!("면적: {}", rect2.area());
    
    let apple = Fruits::Apple;
    let banana = Fruits::Banana;
    let strawberry = Fruits::Strawberry;
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
    
    // let some_number = Some(5);
    // let absent_number: Option<i32> = None;
    
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    
    println!("{:?}", some_number);
    println!("{:?}", absent_number);
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("{:?}, {:?}", six, none);
    
    let some_option = Some(10);
    
    if let Some(val) = some_option {
        println!("값: {}", val);
    } else {
        println!("None");
    }
}
