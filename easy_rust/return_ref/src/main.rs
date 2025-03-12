//OWNERSHIP - 소유권 


fn return_it() -> &'static String {
    let country = String::from("대한민국");
    //본 데이터가 사라지는데 레퍼런스만 리턴하니까 에러 발생 
    //위의 데이터 예시
    &counrty // return &String
}


fn main() {
    
    //하단 변수들은 잘 작동함
    //& = reference 
    // let country = String::from("대한민국");
    // let ref_one = &country;
    // let ref_two = &country;
    
    
    // println!("Country is : {}", ref_one);
    // println!("country is : {}", ref_two);
    
    let my_country = return_it();
    
}
