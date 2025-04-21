//closure 
//anonymous function 
//zero cost abstractions 
//제로 비용 추상화 = 컴파일 타임에 최적화되어 런타임 오버헤드가 발생하지 않는 추상화 기법
//.iter().map().filter().inspect().collect()


fn main() {

    // || 보이면 클로저라고 불림 

    let my_number = 9; 
    let anonymous_function = || println!("i am a function");
    let closure = || println!("{my_number}");

    
    anonymous_function();
    closure();

    //let my_vec = vec![8, 9, 10,0];
    let my_vec = vec![8, 9, 10];
    //let my_vec = vec![];
    let fourth = my_vec.get(3).unwrap_or_else(||{
        if my_vec.get(0).is_some() {
            &my_vec[0]
        } else{
            &0
        }
    });

    println!("{fourth}");

}
