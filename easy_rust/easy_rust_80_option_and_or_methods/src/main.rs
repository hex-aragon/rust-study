// and
// or

fn main() {
    let one = true;
    let two = false;
    let three = false;
    let four = false;

    println!("{}", one && three);
    println!("{}", one && two && three && four);
    println!("{}", one  three);
    println!();
    println!("{}", one  three);
    println!("{}", one  two  three || four);

    // let first_try = vec![Some("success!"), None, Some("success!"), Some("success!"), None];
    // let second_try = vec![None, Some("success!"), Some("success!"), Some("success!"), Some("success!")];
    // let third_try = vec![Some("success!"), Some("success!"), Some("success!"), Some("success!"), None];

    let first_try = vec![Some("s!"), None, Some("s!"), Some("s!"), None];
    let second_try = vec![None, Some("t!"), Some("t!"), Some("t!"), Some("t!")];
    let third_try = vec![Some("u!"), Some("u!"), Some("u!"), Some("u!"), None];

    println!();
    for i in 0..first_try.len() {
        println!("{:?}", first_try[i].and(second_try[i].and(third_try[i])));
        println!("----");
        println!("{:?}", first_try[i].or(second_try[i].or(third_try[i])));
    }
}