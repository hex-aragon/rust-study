// bring into scope
use std::cell::RefCell; // RefCell: 런타임에 내부 가변성을 허용하는 스마트 포인터
use std::rc::Rc;        // Rc: 여러 소유자가 데이터를 공유할 수 있게 해주는 참조 카운트 스마트 포인터

#[derive(Debug)]
struct DataContainer {
    data: Rc<RefCell<String>> // Rc로 감싼 RefCell<String>을 저장 (여러 소유자 + 내부 가변성)
}

fn main() {

    let important_data = Rc::new(RefCell::new("Super duper important data".to_string()));
    // "Super duper important data"라는 String을 RefCell로 감싸고, 다시 Rc로 감싸서 여러 곳에서 공유 및 가변 접근 가능하게 함

    let container_1 = DataContainer {
        data: Rc::clone(&important_data) // important_data의 참조 카운트만 증가시켜 저장
    };

    let container_2 = DataContainer {
        data: Rc::clone(&important_data) // important_data의 참조 카운트만 증가시켜 저장
    };

    //container_1.data() = ""
    for _ in 0..10 {
        // *container_1.data.borrow_mut() = String::from("Hi");
        // *container_2.data.borrow_mut() = String::from("Hi There");
        // 위 코드는 데이터를 완전히 덮어쓰는 예시 (주석 처리됨)

        container_1.data.borrow_mut().push('a');
        // container_1이 가진 data(RefCell<String>)에 가변 접근하여 'a'를 추가

        container_2.data.borrow_mut().push('b');
        // container_2가 가진 data(RefCell<String>)에 가변 접근하여 'b'를 추가

        //이미 디레프가 된 상태라 * 디레프할 필요가 없었음 
        // *container_1.data.borrow_mut().push('a');
        // *container_2.data.borrow_mut().push('b');
        // 위 주석은 borrow_mut()이 이미 String에 대한 가변 참조를 반환하므로, * 연산자가 필요 없다는 설명
    }

    println!("{container_1:?}\n{container_2:?}\n{important_data:?}");
    // container_1, container_2, important_data 모두 같은 String 데이터를 가리키므로, 결과가 모두 동일하게 출력됨
}
