use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    //1. 이동(Move)
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}", s1) //주석 해제시 컴파일 에러 
    println!("{}",s2);

    //2. 복사(Copy)
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    //3. 소유권과 함수 
    let s3 = String::from("world");
    takes_ownership(s3);
    //println!("{}", s3) //주석 해제 시 컴파일 에러 

    //4. 참조와 대여 
    let s4 = String::from("hello world");
    let len = calculate_length(&s4);
    println!("The length of  '{}' is {}.", s4, len);

    //5. 가변 참조
    let mut s5 = String::from("hello");
    change(&mut s5);
    println!("Changed string: {}", s5);

    match read_file("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}",e),
    }
    
    let print_hello = || println!("Hello World");
    let mut logged_hello = wrap_logging(print_hello);
    logged_hello();


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_ownership_move(){
            let s1 = String::from("hello");
            let s2 = s1;
            assert_eq!(s2, "hello");
            //s1은 이미 이동되어 사용할 수 없음 
        }

        #[test]
        fn test_wrap_logging(){
            let mut output = Vec::new();
            {
                let print_to_vec = || output.push("Hello, World");
            let mut logged_print = wrap_logging(print_to_vec);
            logged_print();
            }
            
            assert_eq!(output, vec!["Hello, World"]);
        }
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length( s : &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    Ok(contents)
}

//함수 포인터와 클로저 
fn wrap_logging<F>(mut target: F) -> impl FnMut()
where 
    F: FnMut()
{
    move || {
        println!("Logging Start");
        target();
        println!("Logging End");
    }
}