use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::time::Duration;


async fn hello_world() {
    println!("Hi!!");
}

async fn do_something() {
    hello_world().await;
    println!("Finish");
}

#[tokio::main]
async fn main() {
 
 thread::spawn(|| {
     for i in 1..5 {
         println!("새 스레드: {}", i);
         thread::sleep(Duration::from_millis(500));
     }
 });
 
 for i in 1..5 {
     println!("메인 스레드: {}", i);
     thread::sleep(Duration::from_millis(500));
 }
 
  
 let handle = thread::spawn(|| {
     for i in 1..5 {
         println!("handle 스레드: {}", i);
         thread::sleep(Duration::from_millis(500));
     }
 });
 
 handle.join().unwrap();
 
 let v  = vec![1,2,3];
 let handle2 = thread::spawn(move || {
     println!("벡터: {:?}",v);
 });
  
 handle2.join().unwrap();
 //에러 발생 v의 소유권이 스레드로 이동해서 println!("{:?}",v);
 
 
 let (tx, rx) = mpsc::channel();
 thread::spawn(move || {
     let val = String::from("Hi!");
 //send tx
     tx.send(val).unwrap();
 });
 
 //rx received
 let received = rx.recv().unwrap();
 println!("수신: {}", received);
 
 let m = Mutex::new(0);
 let m = Arc::new(m);
 
 let mut handles = vec![];
 
 for _ in 0..5 {
     let m_clone = Arc::clone(&m);
     let handle = thread::spawn(move || {
         let mut num = m_clone.lock().unwrap();
         *num += 1;
         //잠금 해제는 스코프가 끝날 때 자동으로 drop됩니다. 
         
     });
     handles.push(handle);
 }
 
 for h in handles {
     h.join().unwrap();
 }
 
 println!("최종 값: {:?}", m);
 
 let f1 = async {
     let test = "test";
     println!("{}",test);
     
 };
 let f2 = async {
     let test2 = "test2";
     println!("{}", test2);
 };
 
  let f3 = async {
     let test3 = "test3";
     println!("{}", test3);
 };
 futures::join!(f1, f2, f3);
}
