
fn main() {
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); 
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); 
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}",city1 = a, city2 = b); 
     
 }