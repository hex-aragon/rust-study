// references and the dot operator
// .
// . 

struct Item {
    number: u8
}

// . dot operator
impl Item {
    fn compare_number(&self, other_number: u8){
        println!("Are they equal? {}", self.number == other_number);
    }
}

// Deref * 
fn main() {
    let item = Item {
        number: 10
    };

    //let my_number = 10;  // i32 
    // let reference = &my_number;  // &i32 
    
    // //ref 변수에 * 붙여서 dereference
    // println!("Are they the same? {}", my_number == *reference);
    
    // let reference = &item.number; //&u8
    // //reference.thsadf(); 타입 체크 ?  
    // println!("{}", reference == 10u8);
    
    //https://doc.rust-lang.org/nomicon/ == 러스트의 비결?, 구조 ?에대한 책  
    
    let reference_item = &item;
    item.compare_number(10);
    let other_reference_item = &reference_item; // &&Item
    
    reference_item.compare_number(10); //&Item
    other_reference_item.compare_number(10); 
}
