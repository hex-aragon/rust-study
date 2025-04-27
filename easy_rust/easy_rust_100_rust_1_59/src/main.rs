use std::arch::asm; 
use std::thread::available_parallelism;

// .clone() &T -> T 
// .cloned() Option<&T>
// get()


fn main() {

    // let mut x : u64 = 4;
    // unsafe {
    //     asm!(
    //         "mov {tmp}, {x}",
    //         "sh1 {tmp}, 1",
    //         "sh1 {x}, 2",
    //         "add {x}, {tmp}",
    //         x = inout(reg) x, 
    //         tmp = out(reg) _, 
    //     );
    // };
    // assert_eq!(x, 4*6);

    println!("Hello World!");

    let a = available_parallelism().unwrap();
    println!("We have {a} cores");

    let my_num = "99".parse::<u32>().unwrap();
    //let my_num = "99".parse::<u32>().map(|num| &num);
    println!("{my_num}");

    let my_char = char::try_from(4775u32);
    println!("{my_char:?}");

    let my_u8 = u8::try_from('행');
    println!("{my_u8:?}");


}
