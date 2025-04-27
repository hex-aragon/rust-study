//interior mutability
//multiple threads
//Rc<RefCell>
//Arc<Mutex>
//Mutex = mutual exclusion
use std::sync::{Arc, Mutex}; // Arc(Atomic Reference Counted), Mutex(상호 배제) 임포트

//atomic reference counter
//operating system primitives 

//use std::cell::RefCell;
use std::thread; // 스레드 관련 표준 라이브러리 임포트

// 쿨한 트레잇 정의 (예시용)
trait CoolTrait {
    fn cool_function(&self);
}

#[derive(Debug)]
struct OurStruct {
    data: Arc<Mutex<u8>> // 여러 스레드에서 안전하게 공유할 수 있는 Arc<Mutex<u8>>
}

impl CoolTrait for OurStruct {
    fn cool_function(&self){
        *self.data.lock().unwrap() += 1; // Mutex를 잠그고 내부 값을 1 증가
    }
}

fn main() {

    let our_struct = OurStruct {
        data: Arc::new(Mutex::new(0)) // 0으로 초기화된 Mutex를 Arc로 감싸서 생성
    };

    let mut join_vec = vec![]; // 스레드 핸들을 저장할 벡터

    for _ in 0..10 {
        // 10개의 스레드를 생성
        let clone = Arc::clone(&our_struct.data); // Arc의 참조 카운트만 증가(실제 데이터는 공유)

        let join_handle = thread::spawn( move || { // move 클로저로 clone을 소유권 이동
            //*our_struct.data.lock().unwrap() += 1;
            *clone.lock().unwrap() += 1; // Mutex를 잠그고 1 증가
            println!("There are {} owners ", Arc::strong_count(&clone)); // 현재 Arc의 소유자(참조) 수 출력
        });
        join_vec.push(join_handle); // 각 스레드의 핸들을 벡터에 저장
    }

    for handle in join_vec {
        handle.join().unwrap(); // 모든 스레드가 끝날 때까지 대기
    }

    println!("Our struct is now : {our_struct:?}"); // 최종적으로 our_struct의 상태 출력

    //poisoned = can't use 
    // 만약 어떤 스레드가 Mutex를 잠근 상태에서 패닉이 발생하면, Mutex는 "poisoned" 상태가 되어 이후 lock 시 에러가 발생할 수 있음 (설명용 주석)
}