fn main() {
    println!("Hello, world!");

    let x = 5;
    let mut y = 10;
     println!("{}",y);
    y = 15; 

    const MAX_POINTS: u32 = 100_000;

    println!("{}",x);
    println!("{}",y);
    println!("{}", MAX_POINTS);

    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'R';
     println!("{} {} {} {}",a,b,c,d);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let arr: [i32; 3] = [1,2,3];
    // println!("{} {} {}",tup, tup, tup);
    // println!("{} {} {}",arr, arr, arr);
    
    let mut s = String::from("hello");
    println!("{}",s);
    
    s.push_str(", world");
    println!("{}",s);
    
    let slice:&str = &s[0..5];
    println!("{}",slice);

    let x = 10;
    if x > 5 {
        println!("x is greater than 5");

    } else {
        println!("x is less than or equal to 5");
    }

    let y = if x > 5 {10} else {0};
    println!("{}", y);

    loop {
        println!("Infinite loop!");
        break;
    }

    let mut n = 3;
    while n > 0 {
        println!("{}", n);
        n -= 1;
    }

    for i in 1..4 {
        println!("{}", i);
    }

    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }

    fn add(a: i32, b: i32) -> i32 {
        return a + b; 
    }

    fn square(x: i32) -> i32 {
        x * x 
    }

    let result1 = add(5,3);
    let result2 = square(5);
    println!("Result: {}", result1);
    println!("Result: {}", result2);

    #[derive(Debug)] // Debug를 추가
    struct User{
        username:String, 
        email: String, 
        active: bool,
    }

    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };
    println!("{:?}", user1);

    #[derive(Debug)]
   enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    Unknown,
   };

   let home = IpAddr::V4(127,0,0,1);
   let loopback = IpAddr::V6(String::from("::1"));
   let unknown = IpAddr::Unknown;
   println!("{:?}",home);
   println!("{:?}",loopback);
   println!("{:?}",unknown);

   let s3 = String::from("block string");
   {
    let s4 = s3;
    println!("Inside block: {}", s4);
   }
   //println1("Inside block: {}", s3);

   fn calculate_length(s: &String) -> usize {
    s.len()
   }

   let s1 = String::from("hello");
   let len = calculate_length(&s1);
   println!("{}",s1);
   println!("{}",len);
   fn change(s: &mut String){
        s.push_str(", world");
   }
   println!("{}",s1);

   let mut s = String::from("hello");
   change(&mut s);
   println!("{}",s);


   trait Summary {
    fn summarize(&self) -> String;
   }

   struct Article {
    headline: String, 
    content: String,
   }

   impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}...", &self.content[0..50]);
    }
   }

   fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x 
    } else {
        y
    }
   }

   let string1 = String::from("long string is long");
   let result; 
   {
    let string2 = String::from("short");
    result = longest(string1.as_str(), string2.as_str());
   }

   println!("The longest string is: {}", result);

   fn divide(a: f64, b: f64) -> Result<f64, String> {
         if b == 0.0 {
            Err(String::from("Division by zero"));
         } else {
            Ok(a / b)
         }
   }

   fn find_char(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.chars().enumerate() {
        if ch == c {
            return Some(i)
        }
    }
    None 
    
   }
   
}
