//Mutex and RwLock
//Arc has more overhead 
use std::sync::{Mutex, RwLock}; // Mutex(상호 배제), RwLock(읽기/쓰기 락) 임포트

// RefCell 
// Mutex 
// Rc 
// Arc 

fn main() {
    let my_mutex = Mutex::new(5); // Mutex로 감싼 5라는 값을 생성

    // Mutex를 lock해서 내부 데이터에 대한 가변 참조를 얻음
    // 이 시점에서 Mutex는 "locked" 상태가 됨
    let mut mutex_changer = my_mutex.lock().unwrap();

    //*mutex_changer = 6; // lock을 통해 얻은 가변 참조로 내부 값을 6으로 변경 (주석 처리됨)
    //drop(mutex_changer); // drop을 호출해 lock을 명시적으로 해제 (주석 처리됨)
    // drop을 호출하지 않으면, mutex_changer가 스코프를 벗어날 때까지 lock이 유지됨

    let mut other_mutex_changer = my_mutex.try_lock();
    // 이미 lock이 잡혀 있으므로, try_lock()은 Err를 반환

    if let Ok(value) = other_mutex_changer {
        // 만약 lock을 얻었다면(여기서는 실행되지 않음)
        println!("The mutex changer has {value}");
    }  else {
        // 이미 lock이 잡혀 있으므로 이 부분이 실행됨
        println!("Didn't get a lock")
    }

    // drop을 호출하지 않았으므로, Mutex는 아직 lock된 상태
    println!("{my_mutex:?}"); // 결과: Mutex { data: <locked>, poisoned: false, .. }

    // 만약 drop을 호출했다면, Mutex의 내부 데이터(6 또는 5)가 출력됨
    // drop을 호출하지 않으면, Mutex가 아직 lock된 상태이므로 <locked>로 출력됨

    // poisoned = true: lock을 잡은 스레드가 panic으로 비정상 종료된 적이 있다.
    // poisoned = false: 아직 그런 일이 없다(정상 상태).
    // 일반적으로 poisoned: false라면 안심하고 데이터를 사용할 수 있습니다.

    let my_rwlock = RwLock::new(5); // RwLock으로 감싼 5라는 값을 생성
    let read1 = my_rwlock.write().unwrap(); // 쓰기 락을 얻음 (이 시점에서 읽기 락은 불가능)
    let read2 = my_rwlock.read().unwrap();  // 읽기 락을 얻으려 하지만, 이미 쓰기 락이 있으므로 이 줄에서 데드락(무한 대기) 발생

    println!("{read1:?}, {read2:?}");
    // 위 코드는 실제로 실행되지 않음. read2에서 프로그램이 멈춤(데드락)
}
