fn main() {

    println!("Hello, world!");

    let x = 5;
    println!("The value of x is: {}", x);

    let mut y = 10;
    println!("The value of y is: {}", y);
    
    y = 15; 
    println!("The updated value of y is: {}", y);


    const MAX_POINTS : u32 = 100_000;
    println!("The maximum points is:{}", MAX_POINTS);

    let z = "   ";
    let z = z.len();
    println!("The length of z is: {}",z);

        
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_immutable_variable() {
            let x = 5;
            assert_eq!(x, 5);
        }

        #[test]
        fn test_mutable_variable() {
            let mut y = 10;
            y = 15;
            assert_eq!(y, 15);
        }

        #[test]
        fn test_constant(){
            const MAX_POINTS :i32 = 100_000;
            assert_eq!(MAX_POINTS, 100_000);
        }

        #[test]
        fn test_shadowing(){
            let z = "   ";
            let z = z.len();
            assert_eq!(z,3);
        }
    }

}
