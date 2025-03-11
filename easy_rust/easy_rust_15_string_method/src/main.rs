fn main() {
    println!("Hello World!");
    // allocation   어떤 변수에 16바이트 할당 (ex)
    // reallocation 다시 같은 변수에 32바이트 재할당  (ex)
        
        
    //String
    //.capacity
    //.push > 1글자 
    //.push_str > 1글자 이상
    //.pop
    //.with_capacity
    //allocation
    //.function .으로 쓰는 걸 메소드라고 함
    //len는 capacity보다 작거나 같다. 
    
    //let mut my_name = "".to_string();
    let mut my_name = String::with_capacity(26);
    println!("Length is : {} , Capacity is :{}", my_name.len(), my_name.capacity());
    
    my_name.push_str("David");
    println!("Length is : {} , Capacity is :{}", my_name.len(), my_name.capacity());
    
    my_name.push('!');
    println!("Length is : {} , Capacity is :{}", my_name.len(), my_name.capacity());
    
    my_name.push_str("and i live in seoul");
    println!("Length is : {} , Capacity is :{}", my_name.len(), my_name.capacity());
    
    my_name.push('a');
    println!("Length is : {} , Capacity is :{}", my_name.len(), my_name.capacity());
        
    println!("My name is {}", my_name); 
}
