//any

fn main() {
    
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);
    // rev() reverse
    let mut iterator = big_vec.iter().rev();
    assert_eq!(Some(&5), iterator.next());
    assert_eq!(Some(&6), iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());

    let mut counter = 0;
    //let mut big_iter = big_vec.into_iter();
    let mut big_iter = big_vec.into_iter().rev();
    loop {
        counter += 1;
        if big_iter.next() == Some(5) {
            break;
        }
    }

    println!("Final counter is: {}", counter);
    //println!("{:?}", big_vec.iter().rev().any(|&number| number == 5)); //true 
    //short circuit

}
