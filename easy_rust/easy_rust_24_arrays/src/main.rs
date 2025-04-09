//collection types
//array


//buffer 
fn main() {
    println!("Hello, world!");

    // & str 
    let array = ["One", "Two"]; // [&str; 2] 
   // let array2 = ["One", "Two", "Five"]; //[&str; 3]
    let array2 = ["One", "Two"]; //[&str; 3]
  
    println!("Is array the same as array2? {}", array == array2);
    //true false
    
    
    //타입을 알 수 있다. 컴파일러가 알려줌 
    //array.toasdfasd();

    //file buffer 만들 때도 array 많이 만든다. 
    let mut array = [0; 640];
    println!("{:?}", array);
    
    
    let array3 = ["1월", "2월"]; //indexing 
    println!("{:?}", array3[0]);  //zeroth
    
    
    println!("{:?}", array3.get(3));  //값이 없으면 get을 썼을 때 None으로 나온다. 
    //Some, None Option enum 
    
   // println!("{:?}", array3[2]);  //zeroth
}
