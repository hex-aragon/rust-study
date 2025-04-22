//.chars() - iterator of char
//.count() - counts number of items in iterator
//char_indices
//chars().enumerate()
//index indices 

fn main() {
    let big_string = "Hello there, i am a &str";
    big_string.chars().for_each(|c| println!("{c}"));
    println!("big_string has {} characters", big_string.chars().count());

    big_string.char_indices().for_each(|(index, charrrrr)| {
        println!("at index {index} is the char {charrrrr}");
    });

    let my_vec = vec![8,9,10];
    //my_vec.iter().for_each(|| println!("We don't care about the number")); >> closure is expected to take 1 argument, but it takes 0 arguments
    my_vec.iter().for_each(|_| println!("We don't care about the number")); //위와 같은 에러가 나올 때 아규먼트 처리하려면 이런식으로 수정 필요
    //toilet closure rust 
    //|_| , ::<> (turbo fish)
    //Cow > ? 

}
