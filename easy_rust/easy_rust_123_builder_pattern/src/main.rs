// 빌더 패턴 예제
// 빌더 패턴은 복잡한 객체를 단계적으로 생성할 수 있게 해주는 디자인 패턴입니다

// Character 구조체 정의
// 캐릭터의 기본 정보를 저장
#[derive(Debug)]
struct Character {
    name: String,      // 캐릭터 이름
    age: u8,          // 나이
    height: u32,      // 키
    weight: u32,      // 몸무게
    lifestate: LifeState // 생존 상태
}

// LifeState 열거형 정의
// 캐릭터의 생존 상태를 나타냄
#[derive(Debug)]
enum LifeState {
    Alive,        // 살아있음
    Dead,         // 죽음
    NeverAlive,   // 살아본 적 없음
    Uncertain     // 불확실
}

// Character 구조체의 기본 구현
impl Character {
    // 새로운 Character 인스턴스를 생성하는 연관 함수
    fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
        Self {
            name,
            age,
            height,
            weight,
            // alive 값에 따라 LifeState 결정
            lifestate: if alive { LifeState::Alive } else { LifeState::Dead },
        }
    }
}

// Default 트레이트 구현
// Character의 기본값을 제공
impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),  // 기본 이름
            age: 15,                    // 기본 나이
            height: 170,                // 기본 키
            weight: 70,                 // 기본 몸무게
            lifestate: LifeState::Alive // 기본 생존 상태
        }
    }
}

// 빌더 패턴 구현
// 각 메서드는 self를 소유권으로 받아서 수정 후 반환
impl Character {
    // 나이를 설정하는 빌더 메서드
    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    // 몸무게를 설정하는 빌더 메서드
    fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    // 키를 설정하는 빌더 메서드
    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    // 이름을 설정하는 빌더 메서드
    fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    // 주석 처리된 build 메서드
    // 유효성 검사를 수행하는 최종 빌드 메서드
    // fn build(mut self) -> Result<Character, String> {
    //     if self.height < 200 && self.weight < 300 && 
    // }
}

fn main() {
    // 빌더 패턴을 사용하여 Character 인스턴스 생성
    // 메서드 체이닝을 통해 단계적으로 속성 설정
    let npc_1 = Character::default()    // 기본값으로 시작
        .with_age(20)                   // 나이 설정
        .with_height(194)               // 키 설정
        .with_weight(98)                // 몸무게 설정
        .with_name("Heh I am Smurf");   // 이름 설정
        //.build();                      // 최종 빌드 (현재 주석 처리됨)
    
    println!("{npc_1:?}");  // 생성된 Character 인스턴스 출력
}
