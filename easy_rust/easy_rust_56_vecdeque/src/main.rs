use std::collections::VecDeque;
//Vec Deque 
//vec보다 빠른 경우가 있다. 
//어떤 경우에 ?? 


fn main() {
 //   let my_vec = vec![8, 9, 10, 29]; // .pop, .push() Vec::with_capacity(10)
 //   다음코드로 실행시 느리다 ㄷㄷ 
 //   let mut my_vec = vec![0; 60000];
 //vec deque 사용시 0.56초 정도 걸림, 하단 코드 실행시  
 let mut my_vec = VecDeque::from(vec![0; 600_000]);
 for i in 0..600000 {
     //my_vec.remove(0);
     //println!("{}",i);
     my_vec.pop_front();
 }
 
}