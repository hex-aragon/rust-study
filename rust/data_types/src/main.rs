fn main() {
    println!("Hello, world!");

    //정수형
    let x : i32 = 42;
    println!("x :{}", x);

    //부동 소수점 
    let y : f64 = 3.14;
    println!("y: {}", y);

    //boolean 
    let t : bool = true;
    println!("t: {}", t);

    //문자 
    let c : char = '🦀';
    println!("c: {}",c);

    //튜플
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    let (a,b,d) = tup; 
    println!("a: {}, b: {}, d: {}", a,b,d);

    //배열
    let arr: [i32; 5] = [1,2,3,4,5];
    println!("arr[0]: {}, arr[4]: {}", arr[0], arr[4]);

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_integer() {
            let x : i32 = 42; 
            assert_eq!(x, 42);
        }

        #[test]
        fn test_float(){
            let y: f64 = 3.14;
            assert!((y - 3.14).abs() < f64::EPSILON);
        }

        #[test]
        fn test_char() {
            let c : char = '🦀';
            assert_eq!(c, '🦀');
        }

        #[test]
        fn test_tuple() {
            let tup: (i32, f64, char) = (500, 6.4, 'A');
            assert_eq!(tup.0, 500);
            assert!((tup.1 - 6.4).abs() < f64::EPSILON);
            assert_eq!(tup.2, 'A');
        }

        #[test]
        fn test_array(){
            let arr : [i32; 5] = [1,2,3,4,5];
            assert_eq!(arr[0], 1);
            assert_eq!(arr[4], 5);
        }
    }

}
