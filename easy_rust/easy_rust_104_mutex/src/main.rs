//interior mutability
//multiple threads
//Rc<RefCell>
//Arc<Mutex>
//Mutex = mutual exclusion
use std::sync::Mutex; // Mutex(뮤텍스, 상호 배제)를 사용하기 위해 임포트

//use std::cell::RefCell;
use std::thread; // 스레드 관련 표준 라이브러리 임포트

// 쿨한 트레잇 정의 (예시용)
trait CoolTrait {
    fn cool_function(&self);
}

// Mutex<u8>을 필드로 가지는 구조체
struct OurStruct {
    data: Mutex<u8> // 내부 데이터를 뮤텍스로 감싸서 여러 스레드에서 안전하게 접근 가능
}

// OurStruct에 대해 CoolTrait 구현
impl CoolTrait for OurStruct {
    fn cool_function(&self){
        //self.data += 1; 
        // 위 코드는 Mutex로 감싸져 있으므로 직접 += 연산 불가
        *self.data.lock().unwrap() += 1; // lock()으로 뮤텍스를 잠그고, 내부 값을 1 증가
    }
}

fn main() {

    let our_struct = OurStruct {
        data: Mutex::new(0) // 초기값 0으로 Mutex 생성
    };

    let mut join_vec = vec![]; // 스레드 핸들을 저장할 벡터

    for _ in 0..10 {
        // 10개의 스레드를 생성
        let join_handle = thread::spawn( move || { // move 클로저로 our_struct를 소유권 이동
            *our_struct.data.lock().unwrap() += 1; // 각 스레드에서 Mutex를 잠그고 1씩 증가
        });
        join_vec.push(join_handle); // 각 스레드의 핸들을 벡터에 저장
    }

    for handle in join_vec {
        handle.join().unwrap(); // 모든 스레드가 끝날 때까지 대기
    }

}
