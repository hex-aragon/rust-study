// 'static 특별한 lifetime 

struct Book<'a> { //Generics T, U
    name : &'a  str, 
    second_name: &'a str 
}

struct Adventurer<'a> {
    name: &'a str, 
    hit_points: u32 
}

// implicit == not said 
// elided == not shown 
// 어떤 라이프타임인지 쓸지 정했기 때문에 _ 씀 
// 러스트에서 <'_> 이런 표현 많음 
impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);

    }
}

fn main() { 
    let my_book_title = "my_book_title".to_string();


}
