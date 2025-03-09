#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U>{
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self)-> &T{
        &self.x
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max 
}

pub trait Summary {
    fn summarize(&self) -> String;
    //Summary를 구현하는 구조체는 summarize 메소드를 반드시 구현해야함 
}

struct NewsArticle {
    headline: String,
    content: String, 
}

struct Tweet {
    username: String,
    content: String
}

//Summary 트레잇은 summarize를 무조건 구현해야함!!
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("뉴스기사 - 제목: {}: 내용: {}", self.headline, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("트윗 - 유저 : {}, 내용: {}", self.username, self.content)
    }
}

//트레잇 객체를 매개변수로 받는 함수
fn notify(item: &dyn Summary) {
    println!("{}", item.summarize());
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("가장 큰 숫자: {}", largest(&numbers));
    
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("가장 큰 문자: {}", largest(&chars));
    
    let int_point = Point {x: 5, y: 10};
    let float_point = Point {x: 1.0, y: 4.0}; //제네릭 타입을 컴파일러가 추론해준다. 
    println!("{:?}", int_point);
    println!("{:?}", float_point);
    
    let p = Point2 {x: 5, y: 4.0 };
    println!("{:?}",p);
    
    let article = NewsArticle {
        headline: String::from("뉴스 제목"),
        content: String::from("뉴스 내용"),
    };
    
    let tweet = Tweet{
        username: String::from("유저 이름"),
        content: String::from("트윗 내용"),
    };
    
    //함수 매개변수로 Summary 트레잇을 전달 
    notify(&article);
    notify(&tweet);
}
