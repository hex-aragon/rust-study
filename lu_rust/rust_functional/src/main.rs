fn map_with_fold<T, U, F >(vec: Vec<T>, f: F) -> Vec<U>
where 
    F: Fn(T) -> U
    {
        vec.into_iter().fold(Vec::new(), |mut acc, x|{
            acc.push(f(x));
            acc 
        })
    }

fn filter_with_fold<T, F>(vec: Vec<T>, predicate: F) -> Vec<T>
where 
    F: Fn(&T) -> bool
    {
        vec.into_iter().fold(Vec::new(), |mut acc, x|{
            if predicate(&x){
                acc.push(x);
            }
            acc
        })
    }

fn main() {
    let numbers = vec![1,2,3,4,5];

    let squared = map_with_fold(numbers.clone(), |x| x * x);
    println!("Squared: {:?}", squared);

    let even = filter_with_fold(numbers, |&x| x % 2 == 0);
    println!("Evens {:?}", even);

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_map_with_fold(){
            let numbers = vec![1,2,3,4,5];
            let squared = map_with_fold(numbers, |x| x * x);
            assert_eq!(squared, vec![1,4,9,16,25]);
        }

        #[test]
        fn test_filter_with_fold(){
            let numbers = vec![1,2,3,4,5];
            let evens = filter_with_fold(numbers, |&x| x % 2 == 0);
            assert_eq!(evens, vec![2,4]);
        }
    }
}
