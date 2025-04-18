fn number() -> i32 {
    15
}

fn number_test() -> i32 {
    //10; // () 반환
    10
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}

fn main() {
   
   println!("Hello, world");
   println!("Hello, world number {}!", 8);
   println!("Hello, world number {} and {}!", 8, 9);
   println!("Hello, world number {} ", number());
   //println!("Hello, world number {} ", number_test()); //()를 반환해서 에러 발생
   
   multiply(8,8); // We can give the numbers directly
   let some_number = 10;
   let some_other_number = 2;
   multiply(some_number, some_other_number); // and put them in the function
   
   let my_number = 9;
   println!("Hello, number {}", my_number);
   {
       let my_number2 = 13;
   }
   //println!("Hello, my_number2 {}", my_number2); >> error 
   //cannot find value `my_number2` in this scope


    let my_number3 = {
        let second_number = 8;
        second_number + 34
    };
    println!("My number 3 is :{}", my_number3);
    
   let my_number4 = {
        let second_number = 8;
        second_number + 34;
    };
    //println!("My my_number4 is :{}", my_number4); >> error
    println!("My my_number4 is :{:?}", my_number4); 
    
    
}
