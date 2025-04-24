// any true / false 
// all true / false 

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!("Is {check} inside? {}", char_vec.iter().any(|&character| character == check ));
}

fn main() {
    
    //0..10
    //'a'..='i'
    let char_vec = ('a'..'働').collect::<Vec<char>>();
    println!("{}", char_vec.iter().count());
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '鑿');

    // short-circuiting
    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    println!("All alphabetic? {}", smaller_vec.iter().all(|&character | character.is_alphabetic()));
    println!("All less than the character 행? {}", smaller_vec.iter().all(|&c | c < '행'));
    println!("{smaller_vec:?}");
    //특수문자가 나오는 이유는 all 함수덕분에 알파벳 외에 특수문자가 나오는 것 

}