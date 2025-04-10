// Slices
// Vecs

// dynamically sized type 
 
//Array, Vec
//&str, String=>> Vec 타입, Vec<u8>, Vec<String>

fn main() {
    let seasons = ["봄", "여름", "가을", "겨울","봄", "여름", "가을", "겨울"];
    println!("{:?}", &seasons[0..2]); //[0],[1]
    println!("{:?}", &seasons[0..=2]); //up to and including
    println!("{:?}", &seasons[..]); //all
    println!("{:?}", &seasons[3..]); //[3] 이후 
    println!("{:?}", &seasons[..=4]); //up to and including
}
