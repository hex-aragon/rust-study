use std::cell::{Cell, RefCell}; // Cell과 RefCell을 가져옴 (RefCell은 여기선 사용하지 않음)

//ownership 소유권, 
//Rc - reference counter
//Interior mutability
//external crate tokio rocket rand 
//trait 
//Python 
//drop Rc<dyn Any>
//Rc<RefCell>
//Arc<Mutex> = thread safe reference Counter
//Arc = Atomic reference counter

// 슈퍼 쿨 트레잇 정의, 기본 구현 제공
trait SuperCoolTrait {
    fn cool_function(&self){
        // 기본 구현 (아무 동작도 하지 않음)
    }
}

// User 구조체 정의, Debug 트레잇 자동 구현
#[derive(Debug)]
struct User {
    id : u32,                  // 사용자 ID
    times_used : Cell<u32>     // 몇 번 사용됐는지 저장 (Cell로 내부 가변성 부여)
}

// User에 대해 SuperCoolTrait 구현
impl SuperCoolTrait for User {
    fn cool_function(&self){
        println!("Now using cool_function"); // 함수 호출 시 메시지 출력
        let times_used = self.times_used.get(); // 현재 times_used 값을 가져옴
        self.times_used.set(times_used + 1);    // 값을 1 증가시켜 다시 저장
    }
}

fn main() {
    // User 인스턴스 생성, id는 89723987, times_used는 0으로 초기화
    let user = User {
        id : 89723987,
        times_used: Cell::new(0)
    };

    // cool_function을 20번 호출 (times_used가 20이 됨)
    for _ in 0..20 {
        user.cool_function();
    }

    // User의 현재 상태를 출력 (id와 times_used 값 확인)
    println!("{user:?}");
}