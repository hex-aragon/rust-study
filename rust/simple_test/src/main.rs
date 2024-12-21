fn operate<F>(a: i32, b: i32, op:F) -> i32
where 
    F: Fn(i32, i32)-> i32, 
{
    op(a,b)
}


fn add(a: i32, b: i32) -> i32 { 
    operate(a,b, |x, y| x+y)
}

fn multiply(a: i32, b: i32) -> i32 {
    operate(a,b, |x, y| x*y)
}

fn main() {
    println!("Hello, world!");

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_add(){
            assert_eq!(add(2,3),5);
            assert_eq!(add(-1,1),0);
            assert_eq!(add(0,0),0);


        }

        #[test]
        fn test_multiply(){
            assert_eq!(multiply(2,3),6);
            assert_eq!(multiply(-1,1),-1);
            assert_eq!(multiply(0,5),0);
        }
    }
}
