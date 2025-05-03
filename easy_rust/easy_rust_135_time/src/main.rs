// 표준 라이브러리에서 시간 관련 타입을 가져옴
use std::time::{Duration, Instant};
// 표준 라이브러리에서 스레드의 sleep 함수를 가져옴
use std::thread::sleep;
// 외부 크레이트 chrono와 time을 가져옴
use chrono::{Local, Utc, DateTime}; // chrono 0.4.40
use time::{OffsetDateTime, format_description::well_known::Rfc3339}; // time 0.3.41

fn main() {
    // 현재 시각을 Instant로 저장 (고해상도 타이머, 경과 시간 측정용)
    let time_1 = Instant::now();

    // 1초 동안 현재 스레드를 멈춤
    sleep(Duration::from_secs(1));

    // let now = Instant::now();
    // println!("{now:?}");

    // time_1에서부터 지금까지 경과한 시간을 출력
    println!("std::time elapsed: {:?}", time_1.elapsed());

    // 1000번 반복하며 문자열을 생성 (CPU를 잠시 바쁘게 만듦)
    for _ in 0..1000 {
        let _ = String::from("I am a String to keep you busy");
    }

    // 위 반복문까지 포함한 전체 경과 시간을 출력
    println!("std::time elapsed after loop: {:?}", time_1.elapsed());

    // 새로운 Instant를 저장 (이 시점의 현재 시각)
    let time_2 = Instant::now();

    // time_1과 time_2 사이의 경과 시간을 출력
    println!("std::time difference: {:?}", time_2 - time_1);

    // --- chrono 예제 ---
    // 현재 로컬 시간과 UTC 시간 구하기
    let local: DateTime<Local> = Local::now();
    let utc: DateTime<Utc> = Utc::now();
    println!("chrono local time: {}", local);
    println!("chrono UTC time: {}", utc);

    // chrono를 이용한 시간 포맷팅
    println!("chrono formatted local: {}", local.format("%Y-%m-%d %H:%M:%S"));

    // --- time 크레이트 예제 ---
    // 현재 UTC 시간 구하기
    let now = OffsetDateTime::now_utc();
    println!("time crate UTC: {}", now);

    // time 크레이트의 RFC3339 포맷으로 출력
    let rfc3339 = now.format(&Rfc3339).unwrap();
    println!("time crate RFC3339: {}", rfc3339);

    // time 크레이트로 특정 시간 연산 (예: 1시간 뒤)
    let one_hour_later = now + time::Duration::HOUR;
    println!("time crate one hour later: {}", one_hour_later);
} 
