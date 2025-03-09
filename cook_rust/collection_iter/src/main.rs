use std::collections::HashMap;

fn main() {

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    
    // 또는 아래와 같이 vec! 매크로를 이용하여 초기화할 수 있습니다.
    let v2 = vec![1, 2, 3];
    println!("{:?}",v2);
    
    // 벡터의 index=2(3번째) 원소에 접근
    match v.get(2) {
    	Some(value) => println!("v[2] = {}", value),
    	None => println!("인덱스 범위를 벗어난 접근"),
    }
    
    let v1 = vec![1,2,3,4,5];
    
    for x in &v1 {
        println!("x: {}", x);
    }
    
    let mut iter = v1.iter();
    while let Some(item) = iter.next(){
        println!("item: {}", item);
    }
    
    let mut s = String::from("Hello");
    s.push_str(", Rust!");
    println!("{}", s); 
    
    
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("{:?}",scores);
    
    match scores.get("Blue") {
        Some(&score) => println!("Blue: {}", score),
        None => println!("Not found"),
    }
    
    let key  = "Blue";
    let value = 10;
    scores.insert(key, value);
    
    println!("key = {}, value = {}", key, value);
    
    let v = vec![10,20,30];
    let mut iter = v.iter();
    
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&30));
    assert_eq!(iter.next(), None);
    
    let mut v = vec![1,2,3];
    
    //불변 참조 이터레이터
    for val in v.iter() {
        println!("불변 {}", val);
    }
    
    //가변 참조 이터레이터
    for val in v.iter_mut() {
        *val *= 2; 
        println!("가변 {}", val);
    }
    
    //소유권 이동 이터레이터 
    for val in v.into_iter() {
        println!("순회 끝나면 소유권 없어짐 : {}", val); //v는 순회가 끝난 후 각 요소의 소유권을 잃어버림 
        
    }
    
    //반복문이 끝난 후 v는 각 요소를 사용할 수 없음 
    
    
    let v1 = vec![1,2,3];
    let mapped = v1.iter().map(|x| x * 2); //map 안 쪽이 좀 복잡한데 (x) => x * 2 느낌으로 보면 됨 
    
    // .map()은 Lazy 연산, 즉 colect 등의 메소드 콜로 최종 소비해줘야 실제로 실행됨 
    let collected: Vec<i32> = mapped.collect();
    assert_eq!(collected, vec![2,4,6]);
    
    //filter 
    //이터레이터가 각 원소중 특정 조건을 만족하는 원소만 남겨둠 
    let v = vec![1,2,3,4,5];
    let evens: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    
    println!("{:?}", evens);
    
    let words = vec!["hello", "rust"];
    let uppercased: Vec<String> = words.into_iter().map(|s| s.to_uppercase()).collect();
    
   // println!("{:?}", words);
    println!("{:?}", uppercased);
    
    let v2 = vec![1,2,3];
    //iter()를 사용하면 원본 컬렉션을 사용할 수 있음 
    for x in &v2 {
        //또는 for x in v.iter()
        println!("x: {}", x);
    }
    println!("v[0]: {}", v2[0]);
    
    
    let mut v3 = vec![1,2,3];
    
    // //가변 참조 이터레이터로 순회
    // for x in v3.iter_mut() {
    //     //v는 이미 iter_mut()로 가변 참조 중이므로, 이 도중에 v를 다시 참조하는 것은 규칙 위반
    //     //따라서 컴파일 에러가 발생한다.
    //     v3.push(4);
    // }
    
    // println!("{:?}",v3)
    
    let mut v3 = vec![1,2,3];
    for x in v3.iter_mut() {
        *x *= 2;
        //앞에 *를 붙이는 이유는 , vec는 참조를 뱉기 때문에 포인터의 값을 조회하는 c의 문법을 생각하면 됨 
        
    }
    
    let mut v4 = vec![10,20,30];
    
    //map, filter가 이터레이터 체인으로 연결됨
    let iter_chain = v4.iter_mut().map(|x| {
        *x += 1;
        x
    }).filter(|x| **x > 20);
    
    //위의 체인을 소비하기 위해 collect 메소드 실행 
    //만약 collect 메소드 실행 전에 v를 어떤 방식으로든 가변 참조하려면 에러가 발생함
    let collected: Vec<&mut i32> = iter_chain.collect();
    
    // collect()메소드가 끝난 여기서야 v에 대한 임대가 끝남 
    
    println!("결과: {:?}", collected); //각 요소는 &mut i32 
    println!("v4: {:?}", v4);
}

pub trait Iterator {
    type Item; 
    fn next(&mut self) -> Option<Self::Item>;
}

