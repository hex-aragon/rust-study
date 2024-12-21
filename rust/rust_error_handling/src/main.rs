use std::fs::File;
use std::io::Read;

fn read_username_from_file() -> Result<String, std::io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main(){
    match read_username_from_file(){
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error:{}", e),
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::Write;

        #[test]
        fn test_read_username_from_file(){
            let mut file = File::create("test_hello.txt").unwrap();
            file.write_all(b"test_username").unwrap();

            //함수 테스트
            let result = read_username_from_file();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "test_username");

            //테스트  파일 삭제
            std::fs::remove_file("test_hello.txt").unwrap();
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic(){
        let v = vec![1,2,3];
        let _ = v[99];
    }
}

