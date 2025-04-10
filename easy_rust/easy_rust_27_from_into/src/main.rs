// trait = 초능력 
// trait를 만드는 방법 from, into 
// This type implements (trait name)
// From, Into

fn main() {
    let my_name = String::from("Dave MacLeod"); //from 메소드면서 trait? 
    
    let my_city: String = "Seoul".into(); // &str 
 
    //my_city.asdfas();
    println!("{}", my_city);
    
    let my_vec = Vec::from([8,9,10]); // [i32; 3]
    println!("{:?}",my_vec);
}
