struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn new(x: T, y: T) -> Self{
        Point{x, y}
    }
}

fn main(){
    let integer_point = Point::new(5,10);
    let float_point = Point::new(1.0,4.0);

    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float point:({}, {}",float_point.x, float_point.y);


    #[cfg(test)]
    mod tests{
        use super::*;
        
        #[test]
        fn test_point_creation(){
            let int_point = Point::new(1,2);
            assert_eq!(int_point.x,1);
            assert_eq!(int_point.y,2);
        }

        #[test]
        fn test_generic_function(){
            fn largest<T: PartialOrd>(list: &[T]) -> &T{
                let mut largest = &list[0];
                for item in list.iter(){
                    if item > largest{
                        largest = item;
                    }
                }
                largest
            }
            let number_list = vec![34,50,25,100,65];
            let result = largest(&number_list);
            assert_eq!(*result, 100);
            
            let char_list = vec!['y', 'm', 'a', 'q'];
            let result = largest(&char_list);
            assert_eq!(*result, 'y');

        }
    }
}