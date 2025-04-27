//clippy 
//linter 코드는 잘 되는데 최적화가 안될 때?? 체크  
//fn print_vec_ref(input: &Vec<i32>) {
fn print_vec_ref(input: &[i32]) { // vec, array 모두 슬라이스로 받을 수 있음
    // input.len() == 0 패턴으로 clippy 테스트시 하단과 같이 is_empty()를 쓰라고
    // clippy가 가이드를 준다. 
    if input.is_empty() { // 슬라이스가 비었는지 체크 (clippy 권장)
        println!("Vec is empty");
    } else {
        for num in input {
            println!("{num}"); // 슬라이스의 각 원소를 출력
        }
    }
}

// cargo run 
// cargo clippy 
fn main() {
    let my_vec = vec![8,9,10]; // 벡터 생성
    print_vec_ref(&my_vec);     // 벡터를 슬라이스로 넘김

    let mut done = false;
    let mut counter = 0; 

    // let mut done 
    while done == false { // clippy는 while !done 또는 while !done으로 쓰라고 권장
        counter += 1;
        if counter > 10 {
            done = true; // counter가 10을 넘으면 done을 true로 변경
        }
    }

    let some_variable = Some(9);
    
    if let Some(number) = some_variable {
        // clippy는 match 대신 if let을 권장 (더 간결)
        println!("We got a {number}");
    }

    // 아래 코드로 동작시 clippy가 위와 같이 if let some()형태로 하라고 가이드 줌 
    // match some_variable {
    //     Some(number) => println!("We got a {number}"),
    //     _ => {}
    // }

    let my_vec2 = vec![8,9,10];
    let my_array = [8,9,10];

    print_vec_ref(&my_vec); // 벡터를 슬라이스로 넘김
    //print_vec_ref(&my_array); // 배열도 슬라이스로 넘길 수 있음 (주석 처리됨)
}