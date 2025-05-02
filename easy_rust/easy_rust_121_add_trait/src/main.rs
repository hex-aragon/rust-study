// Add 트레이트를 사용하기 위한 import
use std::ops::Add;

// 빈 구조체 정의 (예시용)
struct Book;

// Country 구조체 정의
// 국가의 이름, 인구, GDP를 저장
#[derive(Debug)]
struct Country {
    name: String,
    population: u32,
    gdp: u32
}

// + Add
// - Sub
// += AddAssign 

// Country 구조체의 구현
impl Country {
    // 새로운 Country 인스턴스를 생성하는 연관 함수
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp
        }
    }
}

// Add 트레이트 구현
// + 연산자를 사용할 수 있게 함
impl Add for Country {
    // Add 트레이트의 출력 타입을 Self(Country)로 지정
    type Output = Self;

    // 두 Country를 더하는 메서드
    fn add(self, other: Self) -> Self {
        Self {
            // 두 국가의 이름을 " and "로 연결
            name: format!("{} and {}", self.name, other.name),
            // 인구를 더함
            population: self.population + other.population,
            // GDP를 더함
            gdp: self.gdp + other.gdp
        }
    }
}

fn main() {
    // Book 구조체 사용 예시 (주석 처리됨)
    // let my_book = Book;
    // let second_book = Book;
    // let third_item = my_book + second_book; // Add 트레이트가 구현되지 않아 컴파일 에러

    // 세 개의 국가 인스턴스 생성
    let nauru = Country::new("Nauru", 10_670, 160_000_000);
    let vanuatu = Country::new("Vanuatu", 307_815, 820_000_000);
    let micronesia = Country::new("Micronesia", 104_468, 367_000_000);

    // 국가들을 더하고 결과 출력
    // Add 트레이트가 구현되어 있어 + 연산자 사용 가능
    println!("Nauru + Vanuatu + Micronesia = {:?}", nauru + vanuatu + micronesia);
    // 출력 예시: Country { name: "Nauru and Vanuatu and Micronesia", population: 422953, gdp: 1347000000 }
}
