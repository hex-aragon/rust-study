use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    //Vec 예제 
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vec: {:?}", vec);

    //String 예제 
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("String : {}",s);

    //HashMap 예제
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    println!("HashMap: {:?}",map);


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_vec(){
            let mut vec = Vec::new();
            vec.push(1);
            vec.push(2);
            vec.push(3);
            assert_eq!(vec, vec![1,2,3]);
        }

        #[test]
        fn test_string() {
            let mut s = String::from("Hello");
            s.push_str(", world");
            assert_eq!(s, "Hello, world");
        }

        #[test]
        fn test_hashmap(){
            let mut map = HashMap::new();
            map.insert(String::from("Blue"), 10);
            map.insert(String::from("Yellow"), 50);
            assert_eq!(map.get("Blue"), Some(&10));
            assert_eq!(map.get("Yellow"), Some(&50));
        }
    }
}
