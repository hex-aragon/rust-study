struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item>{
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci{
    Fibonacci {curr: 0, next: 1}
}


fn lazy_fibonacci() -> impl Fn(usize) -> u64{
    move |n| {
        let mut a = 0;
        let mut b = 1; 
        for _ in 0..n{
            let temp = a; 
            a = b; 
            b = temp + b;
        }
        a 
    }
}

fn main() {


    let fib = lazy_fibonacci();
    println!("10th fibonacci number:{}", fib(10));

    let numbers = 1..;
    let even_squares = numbers
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .take(5);

    for num in even_squares{
        println!("num {}",num);
    }



    let fib = fibonacci().take(10);
    for num in fib {
        println!("{}", num);
    }

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_fibonacci(){
            let fib: Vec<u64> = fibonacci().take(10).collect();
            assert_eq!(fib, vec![1,1,2,3,5,8,13,21,34,55]);
        }

        #[test]
        fn test_lazy_evaluation(){
            let mut iter = fibonacci();
            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(2));
        }
    }
}
