//Box
//owned data on the heap 

struct SomeStruct {
    name: String, 
    number: u8,
   // data: [u8; 1000]
    data: Box<[u8; 1000]>
}

fn take_thing<T>(thing: T) {

}

fn main() {
    let my_struct = SomeStruct {
        name: "Hi there".to_string(),
        number: 0, 
        //data: [9; 1000] The struct is 1032 bytes
        data: Box::new([9; 1000]) //The struct is 40 bytes
    };

    println!("The struct is {} bytes", std::mem::size_of_val(&my_struct));


    // let my_number = 9; 
    // let my_box = Box::new(my_number);
    // take_thing(my_box);
    // println!("{my_box:?}");
    let my_box = Box::new(9);
    println!("{my_box:?}");
    println!("{}", *my_box);
}