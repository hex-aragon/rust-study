
#[macro_export]
macro_rules! my_vec {
    //매개변수가 없는 경우
    () => {
        Vec::new()
    };
    
    //여러 매개변수를 쉼표로 구분하고 반복 처리 
    ($($x:expr),*) => {
    {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
    };
    
}

// #[link(name = "mylib")]
// extern "C" {
//     fn my_c_function(x: i32) -> i32;
// }


//rust함수나 구조체를 export해서 c/c++에서 호출 가능
#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
	x + 1
}

fn main() {
    let v = my_vec![1,2,3];
    println!("{:?}", v);
    
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
        // let result = my_c_function(10);
        // println!("결과: {}", result);
        
    }
     
    
}
