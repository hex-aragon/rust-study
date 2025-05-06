// 에러 처리 관련 표준 라이브러리와 외부 크레이트 가져오기
use std::error::Error;  // 표준 Error 트레이트
use std::num::ParseIntError;  // 숫자 파싱 에러
use std::fmt::{Formatter, Display};  // 포맷팅을 위한 트레이트
use anyhow;  // 편리한 에러 처리를 위한 크레이트 (주: 실제 사용되지는 않음)
use thiserror_1_0_69; // 구조화된 에러 정의에 유용한 크레이트 (주: 실제 사용되지는 않음)

// 회사 내부에서 정의한 커스텀 에러 타입
#[derive(Debug)]  // Debug 트레이트 자동 구현 (디버그 출력용)
enum CompanyError {
    CouldntConect,    // 연결 실패 에러
    NotEnoughData,    // 데이터 부족 에러
    UserTimeOut       // 사용자 시간 초과 에러
} 

// CompanyError에 대한 추가 메서드 구현
impl CompanyError {
    // 추가 세부 정보를 출력하는 커스텀 메서드
    fn print_extra_detail(&self) {
        println!("Here is all the extra detail: blah blah blah");
    }
}

// Display 트레이트 구현 - 에러 메시지 출력 형식 정의
impl Display for CompanyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "got a CompanyError")  // 간단한 에러 메시지
    }
}

// 기본 에러 타입 정의
#[derive(Debug)]
struct BaseError;

// 에러를 반환하는 함수 - Box<dyn Error>로 다양한 에러 타입을 동적으로 반환
// is_company_error: 어떤 타입의 에러를 반환할지 결정하는 플래그
fn give_error(is_company_error: bool) -> Box<dyn Error> {
    if is_company_error {
        Box::new(CompanyError::CouldntConect)  // CompanyError 타입 반환
    } else {
        Box::new(BaseError)  // BaseError 타입 반환
    }
}

// BaseError에 대한 Display 트레이트 구현
impl Display for BaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "got a BaseError")  // 간단한 에러 메시지
    }
}

// Error 트레이트 구현 - 에러 타입으로 동작하게 함
impl Error for CompanyError {
    // 기본 구현 사용 (추가 세부 정보 없음)
}

// BaseError에 대한 Error 트레이트 구현
impl Error for BaseError {
    // 기본 구현 사용 (추가 세부 정보 없음)
}

// 주석 처리된 함수 - 파싱 에러 처리 예제
// fn try_to_make_number(input: &str, float_input: &str) -> Result<i32, ParseIntError> {
// fn try_to_make_number(input: &str, float_input: &str) -> Result<(), ParseIntError> {
//     let my_number = input.parse::<i32>()?;  // ? 연산자로 에러 전파
//     let other_number = input.parse::<f32>()?;
//     Ok(())
// }

fn main() {
    // 두 가지 다른 타입의 에러 생성
    let error_1 = give_error(true);   // CompanyError 반환
    let error_2 = give_error(false);  // BaseError 반환

    // 디버그 형식으로 에러 출력
    println!("{error_1:?}, {error_2:?}");

    // downcast_ref를 사용해 동적 에러 타입 확인 및 타입별 처리
    if let Some(company_error) = error_1.downcast_ref::<CompanyError>() {
        // CompanyError 타입이면 커스텀 메서드 호출
        company_error.print_extra_detail();
    } else {
        // 다른 타입이면 일반 에러 메시지 출력
        println!("{error_1}");
    }
}