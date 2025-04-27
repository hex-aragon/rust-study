// Rc
// 스마트 포인터?
use std::rc::Rc; // Rc(Reference Counted) 스마트 포인터를 사용하기 위해 임포트

// 사용자 정의 구조체 (현재는 비어 있음)
struct MyStruct {

}

// Rc<MyStruct> // MyStruct를 Rc로 감싸서 여러 소유자가 가질 수 있음
// .new_uninit // (설명용 주석) 아직 초기화되지 않은 Rc를 만들 수 있음
// impl MyStruct {
//     fn new_uninit(&self)
// }

// Rc<String> 타입의 값을 받아 출력하는 함수
fn takes_a_string(input: Rc<String> ) {
    println!("{input}");
}

// Rc<String> 타입의 값을 받아 출력하는 또 다른 함수
fn also_takes_a_string(input: Rc<String>) {
    println!("{input}");
}

//anti-type 
//Vec<u8>
//my_vec.get(); //syntactic sugar 

fn main() {

    let my_string = Rc::new("Hello there".to_string());
    // "Hello there"라는 String을 Rc로 감싸서 여러 소유자가 가질 수 있게 함

    //아주 작은 레퍼런스 카운트를 클론 하는 거고 메모리를 거의 안쓴다고 봐야함 
    //소유자가 하나 더 생긴다?
    // Rc::clone(&my_string)은 실제 데이터를 복사하지 않고, 참조 카운트만 증가시킴

    //takes_a_string(my_string);
    //takes_a_string(my_string.clone());
    //clone이 안 좋을 수 있는 이유 
    //clone을 쓰면 새로 만드는 개념 
    //메모리를 많이 쓸 수 있음 (새로 만들기 때문에)
    // Rc::clone은 참조 카운트만 증가시키고, .clone()은 실제 데이터를 복사할 수 있음(타입에 따라 다름)

    takes_a_string(Rc::clone(&my_string)); // Rc::clone을 사용해 참조 카운트만 증가시켜 전달
    also_takes_a_string(my_string);        // 마지막 소유권을 넘김 (이후 my_string 사용 불가)

    // takes_a_string(my_string.clone());
    // takes_a_string(my_string.clone());
    // 위 코드는 실제 데이터를 복사할 수 있으므로, 메모리 사용이 늘어날 수 있음
}
