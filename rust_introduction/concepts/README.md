# Rust 핵심 문법 정리 — `mem_calcu6` 코드로 배우기

이 문서는 `mem_calcu6/src/main.rs` 코드에 등장하는 Rust 문법과 개념을 잘게 쪼개 설명합니다.
각 섹션마다 **`examples/NN_*.rs`** 가 함께 있습니다 — Rust Playground (https://play.rust-lang.org)에 그대로 붙여넣으면 실행됩니다.

## 빠른 길잡이 (코드의 뼈대)

```
use 선언 ─────────────────────── §1
struct Memory + impl 블록 ────── §7, §8
  ├ HashMap<String, f64> ────── §10
  └ Entry API ────────────────── §10
fn main() ─────────────────────── §15 (stdin), §16 (for)
enum Token ────────────────────── §11
  ├ #[derive(Debug, PartialEq)] §13
  └ 데이터 보유 variant ──────── §11
fn eval_* (재귀 하강 파서) ────── §17
```

## 목차
1. [`use` 선언과 모듈 경로](#1-use-선언과-모듈-경로)
2. [`let` / `let mut` / 섀도잉 (shadowing)](#2-let--let-mut--shadowing)
3. [기본 타입 (i32, usize, f64, bool, char)](#3-기본-타입)
4. [**소유권 (Ownership)** — 가장 중요한 개념](#4-소유권-ownership)
5. [**빌림 (Borrowing)** — `&T` / `&mut T`](#5-빌림-borrowing)
6. [**라이프타임 (Lifetime)** — 참조의 유효 기간](#6-라이프타임-lifetime)
7. [`String` vs `&str`, 문자열 슬라이싱](#7-string-vs-str)
8. [`struct` 정의](#8-struct)
9. [`impl` 블록 — 메서드와 연관 함수](#9-impl-블록)
10. [`HashMap` 과 `Entry` API](#10-hashmap-과-entry-api)
11. [`enum` 과 데이터 보유 variant](#11-enum)
12. [패턴 매칭 (`match`, 가드, 바인딩)](#12-패턴-매칭)
13. [`#[derive(...)]` — 트레이트 자동 구현](#13-derive)
14. [`Option<T>` / `Result<T, E>` / `unwrap`](#14-option--result)
15. [표준 입력 (`stdin().lines()`)](#15-표준-입력)
16. [`Vec<T>`, 슬라이스 `&[T]`, `for` 루프](#16-vec와-슬라이스-for-루프)
17. [반복자(Iterator)와 클로저, `map`, `collect`](#17-반복자와-클로저)
18. [튜플 구조분해 (destructuring assignment)](#18-튜플-구조분해)
19. [매크로 (`println!`, `assert_eq!`, `unreachable!`)](#19-매크로)
20. [재귀 하강 파서 패턴](#20-재귀-하강-파서)
21. [부록: 코드에서 발견된 작은 버그들](#부록-코드에서-발견된-버그)

---

## 1. `use` 선언과 모듈 경로

```rust
use std::{
    collections::{hash_map::Entry, HashMap},
    io::stdin,
};
```

- `std`는 표준 라이브러리.
- `{ }` 안에 여러 항목을 한 번에 가져올 수 있음 (중첩 가능).
- 위 코드는 다음과 같음:
  - `std::collections::hash_map::Entry`
  - `std::collections::HashMap`
  - `std::io::stdin`

> 실행: `examples/01_use.rs`

---

## 2. `let` / `let mut` / shadowing

```rust
let mut memory = Memory::new();   // 가변 — 이후 .add()로 내용이 바뀜
let mut prev_result: f64 = 0.0;   // 타입을 명시해 f64로 고정
```

- **`let`** : 불변 바인딩. 한번 묶이면 다시 대입 불가.
- **`let mut`** : 가변 바인딩. 같은 이름 칸에 다시 대입 가능.
- **shadowing** : 같은 이름으로 새 `let`을 또 쓰면, 새 변수가 옛 변수를 가린다. 타입까지 바뀔 수 있다는 점이 가변 변수와 다름.

코드 안에 자주 등장하는 shadowing:
```rust
let line = line.unwrap();        // for 루프의 line(Result)을 String으로 가림
let memory_name = memory_name.to_string();  // &str을 String으로 가림
```

> 실행: `examples/02_let_mut.rs`

---

## 3. 기본 타입

| 코드 | 타입 | 설명 |
|---|---|---|
| `0.0` | `f64` | 64비트 부동소수점 (기본값) |
| `0` | `i32` | 32비트 정수 (기본값) |
| `index` | `usize` | 포인터 크기 정수, 인덱스/길이 전용 |
| `true / false` | `bool` |  |
| `'+'` | `char` | 유니코드 한 글자 (4바이트) |

`parse()`는 문자열을 숫자로 바꿀 때 사용 — 어떤 숫자 타입으로 파싱할지는 좌변 타입에서 추론:

```rust
let v: f64 = "3.14".parse().unwrap();
```

> 실행: `examples/03_primitives.rs`

---

## 4. 소유권 (Ownership)

Rust의 핵심 규칙 3가지:

1. 모든 값은 **소유자(owner)** 가 정확히 한 명 있다.
2. 소유자가 스코프(`{}`)를 벗어나면 값은 **자동 해제(drop)** 된다.
3. 값을 다른 변수/함수에 넘기면 **소유권이 이동(move)** 한다 — 원래 변수는 더 이상 못 쓴다.

코드에서 보이는 예:

```rust
fn add(&mut self, slot_name: String, prev_result: f64) -> f64 { ... }
//                            ^^^^^^ String을 받음 = 호출자의 String 소유권을 가져옴
```

호출 측:
```rust
let memory_name = memory_name.to_string();   // &str을 복제해 새 String을 만듦
let result = memory.add(memory_name, prev_result);  // memory_name 소유권 넘김
// 이후 memory_name 사용 불가
```

**`Copy` 타입은 예외**: `i32`, `f64`, `bool`, `char`, `usize` 등 작고 단순한 값은
`move` 대신 **복사**된다 — 그래서 `prev_result: f64`는 넘긴 뒤에도 계속 쓸 수 있다.

> 실행: `examples/04_ownership.rs`

---

## 5. 빌림 (Borrowing)

소유권을 뺏지 않고 "잠깐 빌려" 보는 방법.

| 표기 | 의미 | 동시에 몇 개? |
|---|---|---|
| `&T` | 불변 빌림 (읽기 전용) | 여러 개 OK |
| `&mut T` | 가변 빌림 (읽기/쓰기) | 단 하나만 |

- 불변 참조 여러 개 ↔ 가변 참조 하나, **둘은 공존 못함** (동시 읽기/쓰기 차단 = 데이터 경합 방지).

코드의 예:
```rust
fn get(&self, slot_name: &str) -> f64 { ... }       // self를 불변 빌림
fn add(&mut self, slot_name: String, ...) -> f64 { } // self를 가변 빌림
```

```rust
let tokens = Token::split(&line);   // line을 빌려서 토큰화
match &tokens[0] { ... }            // Vec의 첫 원소를 빌려서 매칭
```

> 실행: `examples/05_borrow.rs`

---

## 6. 라이프타임 (Lifetime)

라이프타임 = 참조가 **유효한 기간**. Rust는 댕글링 참조(이미 해제된 메모리를 가리키는 참조)를 컴파일 타임에 막는다.

대부분은 **컴파일러가 자동 추론** 해주므로 직접 적을 일이 적다 (lifetime elision).

직접 명시가 필요한 경우 — 입력 참조 여러 개 중 하나를 반환할 때:

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

`'a`는 "이 함수에 들어온 참조와 반환되는 참조의 유효 범위가 모두 같음" 을 표현.

코드에는 라이프타임이 명시적으로 안 보이지만 — `eval_token(token: &Token, memory: &Memory) -> f64`는
사실 `eval_token<'a, 'b>(token: &'a Token, memory: &'b Memory) -> f64`로 컴파일러가 채워 넣음.

> 실행: `examples/06_lifetime.rs`

---

## 7. `String` vs `&str`

| | `String` | `&str` |
|---|---|---|
| 어디 | 힙(heap) | 어디든(보통 다른 곳을 가리킴) |
| 소유권 | 있음 | 없음 (빌림) |
| 변경 | 가능 (`push_str` 등) | 불가능 |
| 만드는 법 | `String::from("x")`, `"x".to_string()` | `"리터럴"`, `&some_string` |

코드에 등장하는 모든 패턴:

```rust
let memory_name = value[3..].to_string();   // &str → String (복제, 소유권 획득)
let memory_name = memory_name.to_string();  // 또 한 번 복제
memory.get(memory_name)                     // String을 &str로 자동 변환해서 넘김
value.starts_with("mem")                    // &str 메서드
value.ends_with('+')                        // 문자 인자
memory_name.pop()                           // String 끝 글자 제거 (Option<char> 반환)
```

`value[3..]`는 **문자열 슬라이싱** — 바이트 인덱스 3부터 끝까지의 `&str`.
> ⚠️ 한글처럼 멀티바이트 문자가 섞이면 바이트 경계가 안 맞으면 panic. 여기선 ASCII만 들어오니 안전.

> 실행: `examples/07_string_str.rs`

---

## 8. `struct`

```rust
struct Memory {
    slots: HashMap<String, f64>,
}
```

- 이름 있는 필드를 가진 데이터 묶음.
- 필드 가시성은 기본 private (같은 모듈 안에서만 접근).
- 인스턴스 만들기:

```rust
let m = Memory { slots: HashMap::new() };
```

`Self { slots: HashMap::new() }`처럼 `impl` 블록 안에서는 `Self`로 자기 타입을 가리킬 수 있음.

> 실행: `examples/08_struct.rs`

---

## 9. `impl` 블록

```rust
impl Memory {
    fn new() -> Self { ... }                            // 연관 함수
    fn add(&mut self, slot_name: String, ...) -> f64    // 메서드 (가변 빌림)
    fn get(&self, slot_name: &str) -> f64               // 메서드 (불변 빌림)
}
```

- **연관 함수**(`new`): `self` 없음. `Memory::new()` 처럼 `타입::함수` 로 호출.
- **메서드**: 첫 인자가 `self`/`&self`/`&mut self`. `instance.method()` 로 호출.

| 첫 인자 | 의미 |
|---|---|
| `self` | 인스턴스 **소유권을 받음** (호출 후 인스턴스 사용 불가) |
| `&self` | 불변 빌림 (읽기만) |
| `&mut self` | 가변 빌림 (수정 가능) |

> 실행: `examples/09_impl.rs`

---

## 10. `HashMap` 과 `Entry` API

```rust
use std::collections::HashMap;

let mut map: HashMap<String, f64> = HashMap::new();
map.insert(String::from("a"), 1.0);
map.get("a")             // Option<&f64> 반환
   .copied()             // Option<&f64> → Option<f64> (f64는 Copy)
   .unwrap_or(0.0);      // 없으면 기본값
```

**Entry API** — "있으면 갱신, 없으면 삽입"을 키 검색 1회로 처리:

```rust
match self.slots.entry(slot_name) {
    Entry::Occupied(mut entry) => {
        *entry.get_mut() += prev_result;  // *로 역참조해서 수정
        *entry.get()
    }
    Entry::Vacant(entry) => {
        entry.insert(prev_result);
        prev_result
    }
}
```

`*entry.get_mut() += x` — `get_mut()`은 `&mut f64`를 돌려주므로 `*` 로 풀어서 값 자체에 더한다.

> 실행: `examples/10_hashmap.rs`

---

## 11. `enum`

```rust
enum Token {
    Number(f64),         // 데이터 1개 (튜플 형태)
    MemoryRef(String),   // String 데이터 1개
    Plus,                // 데이터 없음
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}
```

C의 enum과 달리 **각 variant가 데이터를 가질 수 있다**. 합 타입(sum type) — "이것 또는 저것 중 하나".

만들기 / 매칭:
```rust
let t = Token::Number(3.14);
let t = Token::MemoryRef(String::from("x"));

match &t {
    Token::Number(v)        => println!("{}", v),
    Token::MemoryRef(name)  => println!("{}", name),
    _                       => {}
}
```

> 실행: `examples/11_enum.rs`

---

## 12. 패턴 매칭

```rust
match &tokens[0] {
    Token::MemoryPlus(memory_name) => { ... }      // variant + 내부 String 바인딩
    Token::MemoryMinus(memory_name) => { ... }
    _ => { ... }                                   // 나머지 전부
}
```

특징:
- **모든 경우를 빠짐없이** 다뤄야 컴파일됨 (exhaustiveness).
- `Token::MemoryPlus(memory_name)`처럼 데이터를 변수에 묶을 수 있음.
- `_`는 와일드카드, "그 외 전부".
- **가드(guard)**: `_ if value.starts_with("mem") =>` 처럼 `if` 조건 추가 가능.

코드의 또 다른 예 (`Token::parse`):
```rust
match value {
    "+" => Self::Plus,
    "-" => Self::Minus,
    "*" => Self::Asterisk,
    "/" => Self::Slash,
    _ if value.starts_with("mem") => { ... }   // 가드
    "(" => Self::LParen,
    ")" => Self::RParen,
    _ => Self::Number(value.parse().unwrap()),
}
```

> 실행: `examples/12_match.rs`

---

## 13. `#[derive(...)]`

```rust
#[derive(Debug, PartialEq)]
enum Token { ... }
```

특정 트레이트(인터페이스)를 컴파일러가 자동으로 구현해준다.

| 트레이트 | 효과 |
|---|---|
| `Debug` | `println!("{:?}", x)` 가능 |
| `PartialEq` | `==`, `!=` 비교 가능 (코드에서 `assert_eq!(Token::RParen, tokens[next])` 에 필요) |
| `Clone` | `.clone()` 메서드 사용 가능 |
| `Copy` | 대입할 때 move 대신 복사 |

> 실행: `examples/13_derive.rs`

---

## 14. `Option` / `Result`

| 타입 | variant | 의미 |
|---|---|---|
| `Option<T>` | `Some(T)` / `None` | 값이 있을 수도 없을 수도 |
| `Result<T, E>` | `Ok(T)` / `Err(E)` | 성공 또는 실패 |

코드에서:
```rust
self.slots.get(slot_name)        // Option<&f64>
    .copied()                    // Option<f64>
    .unwrap_or(0.0)              // 없으면 0.0

line.unwrap()                    // Result<String, io::Error> → String (실패 시 panic)
value.parse().unwrap()           // Result<f64, _> → f64

memory_name.pop()                // Option<char> 반환 — 결과는 무시됨
```

`unwrap` 대신 더 안전한 처리 — `match`, `if let`, `?` 연산자 등이 있다.

> 실행: `examples/14_option_result.rs`

---

## 15. 표준 입력

```rust
use std::io::stdin;

for line in stdin().lines() {
    let line = line.unwrap();
    if line.is_empty() { break; }
    // ...
}
```

- `stdin()` — 표준 입력 핸들.
- `.lines()` — 한 줄씩 `Result<String, io::Error>` 를 돌려주는 **Iterator**.
- 줄바꿈 문자(`\n`)는 자동 제거됨.

> 실행: `examples/15_stdin.rs` (Playground는 stdin 입력 가능)

---

## 16. `Vec`와 슬라이스, `for` 루프

```rust
let tokens: Vec<Token> = Token::split(&line);   // Vec<T>: 가변 동적 배열
&tokens[0]                                       // 인덱싱 (참조)
tokens.len()                                     // 길이
&tokens[index..]                                 // 슬라이스 &[Token]
```

함수 인자로 받을 때는 보통 `&[T]`(슬라이스)로 받는 게 유연함:
```rust
fn eval_additive_expression(tokens: &[Token], ...) { ... }
```
→ `Vec<T>`도 `&[T]`로 자동 변환되어 들어감.

`for` 루프는 IntoIterator를 받음:
```rust
for line in stdin().lines() { ... }
for x in &vec { ... }       // &Vec<T> → 원소를 빌림
```

> 실행: `examples/16_vec_for.rs`

---

## 17. 반복자와 클로저

```rust
text.split(char::is_whitespace)   // split('+')처럼 함수도 인자로 받음
    .map(Self::parse)             // 각 토큰을 파싱
    .collect()                    // 결과를 Vec<Self>로 모음
```

핵심 포인트:
- **lazy** : iterator는 `collect`/`for`/`sum` 같은 소비자가 호출되기 전엔 실행 안 됨.
- `map`은 "각 원소에 함수 적용". 인자는 클로저(`|x| x + 1`) 또는 함수 이름(`Self::parse`).
- `collect`는 "어떤 컬렉션으로 모을지" 좌변 타입에서 추론 — `Vec<Self>`로 정해진다.

`char::is_whitespace`는 `fn(char) -> bool` 타입. `split`이 받는 인자는 `Pattern` 트레이트 구현이면 무엇이든 OK.

> 실행: `examples/17_iterators.rs`

---

## 18. 튜플 구조분해

```rust
fn eval_multiplicative_expression(...) -> (f64, usize) { ... }
//                                ^^^^^^^^^^^ 튜플 반환

let (result, next) = eval_multiplicative_expression(...);   // 한번에 두 변수로
```

**Destructuring assignment** (Rust 1.59+):

```rust
let mut index = 0;
let mut result;                          // 선언만
(result, index) = eval_multiplicative_expression(tokens, index, memory);
//   ^^ 둘 다 mut 여야 함, let 없이 기존 변수에 대입
```

> 실행: `examples/18_tuple.rs`

---

## 19. 매크로

매크로는 이름 끝에 `!`가 붙는다 — 함수가 아닌 코드 생성기.

| 매크로 | 의미 |
|---|---|
| `println!("{} {:?}", a, b)` | 형식 문자열 컴파일 타임 검증 후 출력 |
| `assert_eq!(a, b)` | 다르면 panic + 둘 다 출력 |
| `unreachable!()` | "여긴 절대 안 옴" — 도달하면 panic |
| `vec![1, 2, 3]` | `Vec<T>` 리터럴 |
| `panic!("...")` | 즉시 종료 |
| `todo!()` | 미구현 표시 |

`Display` 트레이트가 구현된 타입은 `{}`로, `Debug`만 구현된 타입은 `{:?}` 로 출력.

> 실행: `examples/19_macros.rs`

---

## 20. 재귀 하강 파서

`mem_calcu6`의 핵심 알고리즘. 연산자 우선순위를 함수 호출 깊이로 표현.

```
expression  = additive
additive    = multiplicative (('+'|'-') multiplicative)*
multiplicative = primary (('*'|'/') primary)*
primary     = number | mem_ref | '(' additive ')'
```

각 함수는 `(계산값, 다음 토큰 인덱스)` 튜플을 반환 — 호출자는 `next`를 받아 이어서 진행.

장점:
- 우선순위 자연스럽게 처리 (`*`/`/`가 깊은 함수 → 먼저 묶임).
- 괄호는 `primary`에서 다시 `additive`를 부르며 재귀.

축약된 미니 버전을 `examples/20_parser.rs`에서 실행해 볼 수 있음.

> 실행: `examples/20_parser.rs`

---

## 부록: 코드에서 발견된 버그

학습 차원에서 짚어두면 좋은 두 군데:

### 버그 1 — `unreachable()` (괄호) vs `unreachable!()` (느낌표)

```rust
fn eval_primary_expression(...) -> (f64, usize) {
    match first_token {
        ...
        _ => {
            unreachable();   // ❌ 함수 호출처럼 적힘 — 컴파일 에러
        }
    }
}
```

→ `unreachable!()` 로 고쳐야 함 (매크로니까 `!` 필수).

### 버그 2 — 매개변수 `index`가 즉시 덮어씌워짐

```rust
fn eval_additive_expression(
    tokens: &[Token],
    index: usize,        // ← 이 값을
    memory: &Memory,
) -> (f64, usize) {
    let mut index = 0;   // ❌ 0으로 덮어씀 — 호출자가 넘긴 index가 무시됨
    ...
}
```

`eval_primary_expression`이 괄호 안의 식을 평가할 때 `eval_additive_expression(tokens, index + 1, memory)`로 호출하지만, 함수 안에서 `index = 0`이 되어버려 항상 토큰 0번부터 다시 파싱하게 된다.

→ `let mut index = index;` 로 고쳐야 함 (매개변수를 가변으로 받기).

수정된 두 줄:
```rust
let mut index = index;            // ← 매개변수 그대로 사용
// ...
unreachable!()                    // ← 매크로 호출
```
