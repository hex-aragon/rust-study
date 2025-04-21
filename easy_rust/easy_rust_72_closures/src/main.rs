//closure = anonymous functions that capture their environment
//a | nonymous = no name 
//enclose 
//|| - pipe 
//()

// fn do_it(input : i32) {
// }

// .iter().map(|item| {
// let my_number = 7;
// item + my_number
// }).collect()


fn main() {
    // do_it(8);

    let my_number = 10; 
    let my_closure = |x : i32| println!("{}", x + my_number);
    let my_closure2 = |x : i32| println!("{x}");
    
    my_closure(9);
    my_closure2(5);


    let my_closure3 = || {
        let my_number = 8;
        let other_number = 10; 
        println!("The two numbers are {my_number} and {other_number}");
        my_number + other_number
    };

    let my_var = my_closure3();
    println!("{my_var}");
}
