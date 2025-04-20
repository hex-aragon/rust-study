//associated = goes together
//the type that goes together with the trait 

fn main() {
    let my_vec = vec!['a', 'b', '거', '柳'];
    let mut my_vec_iter = my_vec.iter();
    println!("{:?}", my_vec_iter.next());
    println!("{:?}", my_vec_iter.next());
    println!("{:?}", my_vec_iter.next());
    println!("{:?}", my_vec_iter.next());
 
    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'柳'));
    assert_eq!(my_vec_iter.next(), None);
 
    let a = [1,2,3];
    let mut iter = a.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
 
 
 }
 