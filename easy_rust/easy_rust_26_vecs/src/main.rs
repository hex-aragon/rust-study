// [u8; 10];
// Vec<String>
// Vec<u8>
// T = Some Type
// generics 


// 0 
// reallocation
// 
fn main() {


    //let my_string = String::new();
    //let mut my_vec = Vec::new();
    
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    
    let mut my_vec = Vec::new();
    //push하면서 어떤 타입인지 알 수 있음 위와 같이만 선언하면 컴파일러는
    //알 수 없다. 
    
    println!("Space for my_vec: {}", my_vec.capacity() );
    my_vec.push(name1);
    println!("Space for my_vec: {}", my_vec.capacity() );
    my_vec.push(name2);
    println!("Space for my_vec: {}", my_vec.capacity() );
    
    
    println!("My cats are {:?}", my_vec);
    
    let name3 = String::from("Windy");
    let name4 = String::from("Gomesy");
    
    //매크로 ??
    let my_vec2 = vec![name3, name4];
    println!("my_vec2 {:?}",my_vec2);
}
