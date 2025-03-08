fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("{}, {}, {}", x, y, z);
    
    let arr: [i32; 3] = [1,2,3];
    println!("{}", arr[0]);
    
    let arr = [1,2,3,4,5];
    let slice = &arr[1..3];
    println!("{:?}", slice);
    
    
    let number = 7;
    if number < 5{
        println!("Number < 5");
    } else if number == 7 {
        println!("Number == 7");
    } else {
        println!("Number > 5 and not 7");
    }
    
    let condition = true;
    let num = if condition {10} else {5};
    
    println!("{}", num);
    
    let x = 2;
    match x {
        1 => println!("Number is 1"),
        2 | 3 => println!("Number is 2 or 3"),
        4..=10 => println!("Number is in 4~10"),
        _=> println!("Or other case..."),
    }
    
    let mut count1 = 0;
    loop {
    	count1 += 1;
    	if count1 == 3 {
    		break;
    	}
    }

    println!("count = {}", count1);   
    
    let mut count = 0;
    
    let result = loop {
        count += 1;
    if count == 3 {
            break 10; 
        }
    };
    println!("result = {}", result);   // Output : 10
    
    
    let mut n = 3;
    while n != 0 {
        println!("{}", n);
        n -= 1;
    }
    
    for i in 1..6 {
        println!("{}", i);
    }
    
    let a = [10, 20, 30, 40, 50];
    for element in a.iter(){
        println!("{}", element);
    }
    
    fn add(x: i32, y: i32) -> i32 {
        x + y 
    }
    println!("{}",add(3,4));
}