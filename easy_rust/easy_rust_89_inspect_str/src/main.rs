// .inspect
// String &str 
struct Book<'a> { //lifetime 
    title: String,
    con : &'a str 
}

//c,c++ 문제 > use after free >> 러스트가 해결한 게 라이프타임 
fn main() { 
    let new_vec = [8,9,10];

    let double_vec = new_vec 
        .iter()
        .inspect(|first_item| {
            dbg!(first_item);
            })
        .map(|x| x * 2)
        .inspect(|next_item| {
            dbg!(next_item);
        })
        .filter(|num| *num > 17) 
        .collect::<Vec<_>>();
    
    dbg!(double_vec);

    let my_book = Book {
        title: "my_title".to_string(),
        con : "test"
    };

}
