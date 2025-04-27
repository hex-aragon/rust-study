//Debug again 
// Debug 트레잇 관련 예제

//External code 
mod client { // client라는 모듈 정의 (외부 코드처럼 사용)
    pub struct InternetClient { // 공개 구조체 InternetClient
        pub client_id : u32  //Other stuff  // 공개 필드 client_id (실제로는 더 많은 필드가 있을 수 있음)
    }
}

use client::InternetClient; // InternetClient를 현재 스코프로 가져옴

// Customer 구조체 정의, 라이프타임 'a를 가짐
struct Customer<'a> {
    money: u32,                    // 고객이 가진 돈
    name: &'a str,                 // 고객 이름 (참조)
    client: &'a InternetClient     // InternetClient에 대한 참조
}

use std::fmt; // 포맷팅 관련 표준 라이브러리 사용

// Customer<'_>에 대해 Debug 트레잇 수동 구현
impl fmt::Debug for Customer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Customer") // "Customer"라는 이름으로 디버그 출력 시작
        .field("money", &self.money) // money 필드 출력
        .field("name", &self.name)   // name 필드 출력
        .field("client", &"Client")  // client 필드는 실제 값 대신 "Client"라는 문자열로 출력
        .finish()                    // 출력 마무리
    }
}

fn main(){
    let client = client::InternetClient {
        client_id:0 // client_id를 0으로 설정한 InternetClient 인스턴스 생성
    };

    let customer1 = Customer{
        money: 6875,         // 돈 6875
        name: "Billy",       // 이름 "Billy"
        client: &client      // 위에서 만든 client에 대한 참조
    };

    println!("{customer1:?}"); // customer1을 Debug 포맷으로 출력
}