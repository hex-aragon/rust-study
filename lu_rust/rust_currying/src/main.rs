fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}



fn main() {

    let add_5 = curry_add(5);
    println!("5 + 3 = {}", add_5(3));
    println!("5 + 7 = {}", add_5(7));

    let add_10 = curry_add(10);
    println!("10 + 3 = {}", add_10(3));
    println!("10 + 7 = {}", add_10(7));

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_curry_add(){
            let add_5 = curry_add(5);
            assert_eq!(add_5(3), 8);
            assert_eq!(add_5(7), 12);

            let add_10 = curry_add(10);
            assert_eq!(add_10(3), 13);
            assert_eq!(add_10(7), 17);
        }
    }
}
