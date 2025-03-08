
// fn takes_ownership(some_string: String) {
//     //println!("{}", some_string);
// }

fn change(str: &mut String) {
    str.push_str(" Rust!"); //가변참조라서 원본수정이 가능함 
}

fn calculate_length(s: &String) -> usize {
   return s.len();
}




fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// fn longest_without_lifetime(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// } 


fn first_word<'a>(s: &'a str) -> &'a str{
    s 
}


// fn longest2(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {x} else {y} //missing lifetime 
// }


fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {x} else {y}
}

struct MyString {
    data: String,
}

impl MyString {
    fn get_data(&self) -> &str {
        &self.data
    }
}
//컴파일러가 해석하는 구문
//fn get_data<'a>(&'a self) -> &'a str 


// fn return_str() -> &str {
//     "hello" //hello 반환, 라이프타임 없어서 에러
// }
fn return_str() -> &'static str {
    "hello" //hello 반환, 라이프타임 없어서 에러
}


fn main() {
    // let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("{}",s1);
    
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    
    // let s = String::from("Hello");
    // takes_ownership(s);
    // println!("{}", s);
    let len = calculate_length(&s1);
    println!("s1 = {}, length = {} ",s1, len);
    
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);
    
    // let r;
    // {
    //     let x = 10;
    //     r = &x;    
        
    // }
    // println!("r: {}", r);
    
    let x = 5;
    let r = &x; 
    println!("r: {}", r);
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {}", result);
    
    // let result2 = longest_without_lifetime(string1.as_str(), string2);
    // println!("더 긴 문자열: {}", result);
    
    // let s1 = "Hi".to_string();
    // let result; 
    
    // {
    //     let s2 = "Rust".to_string();
    //     result = longest(&s1, &s2);
    // }
    // println!("{}", result);
    
    
    let my_string = String::from("Hello Rust");
    let word = first_word(&my_string);
    println!("{word}");
    
    let s1 = "Hello".to_string();
    let s2 = "Rust".to_string();
    let result2 = longest2(&s1, &s2);
    println!("더 긴 문 자열은 = {result2}");
    
    let hello = return_str();
    println!("hello return_str = {hello}");
}
