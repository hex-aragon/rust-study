//match_indices indices = indexes
//peekable == .peek = .next()

fn user_input() -> String {
    //do something 
    "user_name".to_string()
}

fn main() { //enumerate랑 비슷함, 인덱스랑 위치 찾아줌 
    let rules = "Rule number 1 : No fighting.
Rule number 2: Go to bed at 8 pm.
Rule number 3: Wake up at 6 am.";

    let rule_locations = rules.match_indices("Rule").collect::<Vec<(_, _)>>();
    println!("Rule locations : {rule_locations:?}");
    
    let number_locations2 = rules.match_indices("number").collect::<Vec<(_, _)>>();
    println!("Number locations : {number_locations2:?}");
    

    let just_numbers = vec![1,5,100];

    let mut number_iter = just_numbers.iter().peekable();

    //number_iter.ntoasdf(); 타입 체크 
    for _ in 0..3 {
        println!("I love the number {}", number_iter.peek().unwrap());
        println!("I really love the number {}", number_iter.peek().unwrap());
        number_iter.next(); //pop과 비슷 next를 해야 다음 넘버로 넘어감 
    }
}
