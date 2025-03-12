//attribute
#[allow(non_upper_case_globals)]
const high_score: i32 = 20; // global scope

const HIGH_SOCRE1: i32 = 23;

//static 
static  LOW_SCORE: i32 = 0; //unsafe  >> 안좋음, 안 쓰는 게 좋음,
//rust는 안전함을 지향하기 때문에 


//lifetime >> 
//static lifetime >> 
fn print_high_score(){
    println!("The high score is {}", high_score);
}


fn main() {

    let x = 8; // 'let' binding: i32
    println!("let x = {}", x);
    println!("HIGH_SCORE = {}", high_score);
    println!("HIGH_SOCRE1 = {}", HIGH_SOCRE1);
    println!("LOW_SCORE = {}", LOW_SCORE);
    
    print_high_score();
    
    // unsafe {
    //     LOW_SCORE = 1;
    // }
    //  println!("LOW_SCORE mut = {}", LOW_SCORE);
    let my_name = "David";
    //&'static str 
    
    
}
