
fn main() {
   
    let small_number = 10u8;
    let small_number2 : u8 = 10; 
    let small_number3 = 10_u8;
    let big_number = 100_000_000_i32;
    
    println!("small_number is {} ", small_number);
    println!("small_number2 is {} ", small_number2);
    println!("small_number3 is {} ", small_number3);
    println!("big_number is {} ", big_number);
    
    
    let number = 0________u8;
    let number2 = 1___6_____2____4______i32;
    println!("{}, {}", number, number2);
    // small_number is 10 
    // small_number2 is 10 
    // small_number3 is 10 
    // big_number is 100000000 
    // 0, 1624
    
    
    let my_float = 5.;
    let my_float2 = 5.0;
    
    println!("{} {}", my_float, my_float2);
    
    let my_float3 : f64 = 5.0;
    let my_other_float : f32 = 8.5;
    let third_float = my_float3 + my_other_float as f64;
    
    println!("{}, {}, {}", my_float3, my_other_float, third_float);
    
    let my_float4 = 5.0; // Rust will choose f64
    let my_other_float4 = 8.5; // Here again it will choose f64

    let third_float4 = my_float4 + my_other_float4;
    println!("{}", third_float4);
    
    
}
