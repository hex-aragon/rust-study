use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (sender, receiver) = channel();

    // sender.send(8).unwrap();
    // let received  = receiver.recv().unwrap();
    // println!("{received}");

    let s1 = sender.clone();
    let s2 = sender.clone();

    thread::spawn(move || {
        s1.send(8).unwrap();
    });

    thread::spawn(move || {
        s2.send(8).unwrap();
    });

    println!("{}", receiver.recv().unwrap());
    println!("{}", receiver.recv().unwrap());
}

/*
[move 키워드 설명]
- Rust의 클로저(익명 함수)에서 move 키워드를 사용하면,
  클로저가 자신이 사용하는 변수의 소유권을 클로저 내부로 이동시킵니다.
- 이 예제에서는 s1, s2가 각각의 스레드로 move되어,
  각 스레드가 안전하게 채널의 송신자를 소유하고 사용할 수 있습니다.
- 만약 move를 사용하지 않으면, 클로저가 외부 변수의 참조만 가지게 되어
  스레드가 종료될 때까지 변수의 생명주기 문제가 발생할 수 있습니다.
- 스레드와 같이 소유권이 명확해야 하는 상황에서 move는 필수적으로 사용됩니다.
*/ 
