fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
     
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("Longest string is {}", result);

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_longest(){
            let string1  = String::from("long");
            let string2 = String::from("longer");
            assert_eq!(longest(string1.as_str(), string2.as_str()), "longer");

        }

        #[test]
        fn test_lifetime_in_struct(){
            struct ImportantExcerpt<'a>{
                part: &'a str,
            }

            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            let i = ImportantExcerpt{
                part: first_sentence,
            };
            assert_eq!(i.part, "Call me Ishmael");
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}