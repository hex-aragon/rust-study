// Default and the builder pattern
// Default
// native Implementation

// 빌더 패턴의 두 번째 구현 예제
// 별도의 빌더 구조체를 사용하여 객체 생성 과정을 분리

// Character 구조체 정의
// 캐릭터의 기본 정보와 사용 가능 여부를 저장
#[derive(Debug)]
struct Character {
    name: String,      // 캐릭터 이름
    age: u8,          // 나이
    height: u32,      // 키
    weight: u32,      // 몸무게
    lifestate: LifeState, // 생존 상태
    can_use: bool     // 사용 가능 여부 플래그
}

// CharacterBuilder 구조체 정의
// Character 생성 과정을 담당하는 빌더
#[derive(Debug)]
struct CharacterBuilder {
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

// Character 사용 함수
// can_use 플래그가 true인 경우에만 사용 가능
fn use_character(character: &Character) {
    if character.can_use {
        // 함수 구현
    }
}

// Character의 Default 트레이트 구현
impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),  // 기본 이름
            age: 15,                    // 기본 나이
            height: 170,                // 기본 키
            weight: 70,                 // 기본 몸무게
            lifestate: LifeState::Alive, // 기본 생존 상태
            can_use: true               // 기본적으로 사용 가능
        }
    }
}

// CharacterBuilder의 Default 트레이트 구현
impl Default for CharacterBuilder {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),  // 기본 이름
            age: 15,                    // 기본 나이
            height: 170,                // 기본 키
            weight: 70,                 // 기본 몸무게
            lifestate: LifeState::Alive  // 기본 생존 상태
        }
    }
}

// CharacterBuilder 구현
impl CharacterBuilder {
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

    // 최종 Character 객체 생성
    // 유효성 검사를 수행하고 결과 반환
    fn build(self) -> Result<Character, String> {
        // 유효성 검사 조건:
        // 1. 키가 200 미만
        // 2. 몸무게가 300 미만
        // 3. 이름에 "smurf"가 포함되어야 함 (대소문자 구분 없음)
        if self.height < 200 && self.weight < 300 && self.name.to_lowercase().contains("smurf") {
            Ok(Character {
                name: self.name,
                age: self.age,
                height: self.height,
                weight: self.weight,
                lifestate: self.lifestate,
                can_use: true  // 유효성 검사 통과 시 사용 가능
            })
        } else {
            Err("Names must contain Smurf, weight must be under 300, height under 200.".to_string())
        }
    }
}

fn main() {
    // 빌더 패턴을 사용하여 CharacterBuilder 인스턴스 생성
    let npc_1 = CharacterBuilder::default()
        .with_age(20)                   // 나이 설정
        .with_height(56)                // 키 설정
        .with_weight(98)                // 몸무게 설정
        .with_name("Billyrobby smurf"); // 이름 설정
    println!("{npc_1:?}");             // 빌더 상태 출력
    
    // build() 메서드를 호출하여 Character 객체 생성
    let npc_1 = npc_1.build();         // shadowing을 사용하여 결과 저장
    println!("{npc_1:?}");             // 생성된 Character 객체 출력
}