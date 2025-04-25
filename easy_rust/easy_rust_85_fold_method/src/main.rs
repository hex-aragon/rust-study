

fn main() {
    let some_numbers = vec![9, 6, 9, 10, 11];

    println!("{}", some_numbers
        .iter()
        .fold(0, | total_so_far, next_number | total_so_far + next_number
        ));

    let total : i32 = some_numbers.iter().sum();
    println!("{}", some_numbers.iter().sum::<i32>());

    let a_string = "I don't have any dashes in me";

    let dashed = a_string
        .chars() //iterator
        .fold("-".to_string(), |mut string_so_far, next_char|{
            string_so_far.push(next_char); //()
            string_so_far.push('-'); //()
            string_so_far
        });
    println!("{dashed}");

}
