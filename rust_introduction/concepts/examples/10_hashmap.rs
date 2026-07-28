// §10. HashMap과 Entry API

use std::collections::{hash_map::Entry, HashMap};

fn main() {
    // 만들기
    let mut map: HashMap<String, f64> = HashMap::new();

    // 삽입
    map.insert(String::from("a"), 1.0);
    map.insert(String::from("b"), 2.0);

    // get → Option<&V>
    println!("{:?}", map.get("a"));         // Some(1.0)
    println!("{:?}", map.get("missing"));   // None

    // mem_calcu6의 패턴: 없으면 0.0
    let value = map.get("missing").copied().unwrap_or(0.0);
    println!("missing → {}", value);

    // ── Entry API ────────────────────────────────────
    // "있으면 갱신, 없으면 삽입"을 키 검색 1회로 끝낸다.

    // (a) match로 명시적 처리
    match map.entry(String::from("a")) {
        Entry::Occupied(mut e) => {
            *e.get_mut() += 10.0;     // get_mut()는 &mut f64 → *로 역참조 후 수정
            println!("기존 a: {}", e.get());
        }
        Entry::Vacant(e) => {
            e.insert(0.0);
            println!("a 새로 만듦");
        }
    }

    // (b) or_insert로 더 짧게
    *map.entry(String::from("c")).or_insert(0.0) += 100.0;
    *map.entry(String::from("c")).or_insert(0.0) += 100.0;
    println!("c = {:?}", map.get("c"));   // Some(200.0)

    // 전체 출력 (Debug 형식)
    println!("{:?}", map);
}
