
enum Shape {
    Circle {radius: f64},
    Rectangle {width: f64, height: f64},
    Triangle(f64, f64, f64),
}


fn main() {
    let number = 3;
    match number {
        1 => println!("one!!"),
        2 | 3 => println!("two or three!"),
        4..=10 => println!("4-10"),
        _ => println!("others..."),
    }
    
    let x = 10;
    let y = match x {
        0 => "zero",
        1 => "one",
        2 | 3 => "two or three",
        _ => "others...",
    };
    
    println!("{y}");
    
    
    let x = 10;
    match x {
        n if n < 5 => println!("less than 5"),
        n if n % 2 == 0 => println!("even"),
        n if n % 2 == 1 => println!("odd"),
        _ => println!("what?"),
    }
    
    let some_option = Some(10);
    //match를 활용하면 다음과 같은 코드를 구현 가능 
    match some_option {
        Some(val) => println!("값: {}", val),
        None => (),
    }
    
    //if let 을 이용하면 아래와 같이 구현 가능 
    if let Some(val) = some_option {
        println!("값: {}", val);
    } else {
        println!("None");
    }
    
    
    let mut stack = vec![1,2,3];
    while let Some(top) = stack.pop(){
        println!("스택의 값: {}", top);
    }
    
    //_ 와일드가드 패턴, 
    let some_value = 110;
    match some_value {
        0 => println!("0"),
        _ => println!("Not 0"),
    }
    
    //.. 구조체, 튜플 등 일부필드만 매칭할 때 사용 
    struct Point {x: i32, y:i32, z:i32};
    let p = Point {x: 10, y: 20, z:30 };
    
    match p {
        Point {x, ..} => println!("x: {}", x),
    }
    
    //ref 매치 과정에서 값 이동 없이 참조로 빌릴 때 사용 
    let s = String::from("hello");
    match s {
        ref ref_s => println!("참조: {}", ref_s),
    }
    
    let r = &10;
    //&val 패턴으로 r이 가리키는 실제 값을 확인
    match r {
        &val => println!("val: {}", val),
    }
    
    //@바인딩 
    //매칭되는 값을 별도로 바인딩하면서 동시에
    //그 값에 대한 특정 조건도 확인하고 싶을 때 사용한다. 
    let msg = Some(5);
    
    match msg {
        Some(n @ 1..=10) => println!("1~10사이의 수: {}", n),
        Some(n) => println!("그 외의 수: {}", n),
        None => println!("값이 없음"),
    }
    
    let (x, y, z) = (10, 20, 30);
    println!("x={}, y={}, z={}",x,y,z);
    
    struct Point1 {x: i32, y:i32};
    let p = Point1 {x: 5, y: 10};
    
    let Point1 {x:a, y:b} = p;
    println!("a={}, b={}",a,b);
    
    
    let shape = Shape::Circle {radius: 2.5};
    match shape {
        Shape::Circle {radius} => println!("원, 반지름={}", radius),
        Shape::Rectangle {width, height} =>{
            println!("직사각형, 너비: {}, 높이: {}", width, height);
        }
        Shape::Triangle(a,b,c) =>{
            println!("삼각형, 각 변의 길이 = {}, {}, {}",a,b,c);
        }
    }
}
