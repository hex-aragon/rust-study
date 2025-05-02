// Default 트레이트와 빌더 패턴 예제
// Default 트레이트는 타입의 기본값을 제공하는 트레이트입니다

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

// Character 구조체의 구현
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

fn main() {
    // 기본 타입들의 기본값 출력
    println!("{}, {}, {}, {}", 
        i32::default(),    // 0
        String::default(), // ""
        f32::default(),    // 0.0
        char::default()    // '\0'
    );

    // Character 인스턴스 생성 방법 1: new() 사용
    // let npc_1 = Character::new("Billy".to_string(), 15, 170, 70, true);
    // println!("{npc_1:?}");
    
    // Character 인스턴스 생성 방법 2: Default 트레이트 사용
    let npc_1 = Character::default();
    println!("{npc_1:?}");
    // 출력: Character { name: "Billy", age: 15, height: 170, weight: 70, lifestate: Alive }
}
