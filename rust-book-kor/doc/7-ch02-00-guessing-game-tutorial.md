추측 게임 프로그래밍하기
함께 실습 프로젝트를 진행하며 Rust의 세계로 뛰어들어 봅시다! 이 장에서는 몇 가지 일반적인 Rust 개념을 소개하고 실제 프로그램에서 사용하는 방법을 보여줍니다. `__init__`, `__call__` let, match메서드, 연관 함수, 외부 크레이트 등에 대해 배우게 될 것입니다. 다음 장에서는 이러한 개념들을 더 자세히 살펴보겠습니다. 이 장에서는 기본 사항만 연습합니다.

초보자에게 인기 있는 프로그래밍 문제인 추측 게임을 구현해 보겠습니다. 작동 방식은 다음과 같습니다. 프로그램은 1에서 100 사이의 임의의 정수를 생성합니다. 그런 다음 플레이어에게 추측할 값을 입력하라는 메시지를 표시합니다. 추측이 입력되면 프로그램은 해당 값이 너무 낮은지 또는 너무 높은지를 알려줍니다. 추측이 맞으면 축하 메시지를 출력하고 게임을 종료합니다.

새 프로젝트 설정하기
새 프로젝트를 설정하려면 1장에서 생성한 프로젝트 디렉토리로 이동하여 다음과 같이 Cargo를 사용하여 새 프로젝트를 만드세요.

$ cargo new guessing_game
$ cd guessing_game
첫 번째 명령어는 cargo new프로젝트 이름( guessing_game)을 첫 번째 인수로 받습니다. 두 번째 명령어는 새 프로젝트의 디렉터리로 이동합니다.

생성된 Cargo.toml 파일을 살펴보세요 .

파일명: Cargo.toml

[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
1장에서 보셨듯이, 이 프로그램은 "Hello, world!"라는 메시지를 생성합니다. src/main.rscargo new 파일을 확인해 보세요 .

파일 이름: src/main.rs

fn main() {
    println!("Hello, world!");
}
이제 다음 명령어를 사용하여 "Hello, world!" 프로그램을 컴파일하고 동시에 실행해 보겠습니다 cargo run.

$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/guessing_game`
Hello, world!
이 run명령어는 이 게임에서처럼 프로젝트를 빠르게 반복해야 할 때 유용합니다. 각 반복 작업을 신속하게 테스트한 후 다음 반복 작업으로 넘어갈 수 있기 때문입니다.

src/main.rs 파일을 다시 여세요 . 모든 코드는 이 파일에 작성할 겁니다.

추측을 처리하는 중
추측 게임 프로그램의 첫 번째 부분은 사용자 입력을 요청하고, 해당 입력을 처리하고, 입력이 예상되는 형식인지 확인합니다. 우선, 플레이어가 추측을 입력할 수 있도록 하겠습니다. 목록 2-1의 코드를 src/main.rs 에 입력하세요 .

파일 이름: src/main.rs
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
목록 2-1 : 사용자로부터 추측을 입력받아 출력하는 코드
이 코드는 많은 정보를 담고 있으므로 한 줄씩 살펴보겠습니다. 사용자 입력을 받고 결과를 출력하려면 io입출력 라이브러리를 스코프 안으로 가져와야 합니다. 이 io라이브러리는 표준 라이브러리에 포함되어 있으며, 다음과 같이 알려져 있습니다 std.

use std::io;
Rust는 기본적으로 모든 프로그램의 스코프에 포함되는 표준 라이브러리에 정의된 항목 집합을 가지고 있습니다. 이 집합을 ' 프렐류드(prelude)' 라고 하며, 표준 라이브러리 문서에서 그 안에 있는 모든 항목을 확인할 수 있습니다 .

사용하려는 타입이 프리루드에 없으면, 명시적으로 해당 타입을 스코프 내로 가져와야 합니다 use. 이 std::io라이브러리를 사용하면 사용자 입력을 받는 기능을 포함하여 여러 유용한 기능을 활용할 수 있습니다.

1장에서 보셨듯이, 함수 main는 프로그램의 진입점입니다.

fn main() {
이 fn구문은 새로운 함수를 선언합니다. 괄호("")는 ()매개변수가 없음을 나타내고, 중괄호("" {)는 함수의 본문을 시작합니다.

1장에서 배웠듯이, 는 println!화면에 문자열을 출력하는 매크로입니다.

    println!("Guess the number!");

    println!("Please input your guess.");
이 코드는 게임이 무엇인지 알려주는 메시지를 출력하고 사용자로부터 입력을 요청합니다.

변수를 사용하여 값 저장하기
다음으로, 사용자 입력을 저장할 변수를 다음과 같이 생성하겠습니다 .

    let mut guess = String::new();
이제 프로그램이 흥미로워지네요! 이 짧은 코드 한 줄에 많은 내용이 담겨 있습니다. 우리는 이 let구문을 사용하여 변수를 생성합니다. 또 다른 예시를 보여드리겠습니다.

let apples = 5;
이 코드는 라는 이름의 새 변수를 생성 apples하고 값 를 할당합니다 5. Rust에서 변수는 기본적으로 불변(immutable)입니다. 즉, 변수에 값을 할당하면 해당 값은 변경되지 않습니다. 이 개념은 3장의 "변수와 가변성" 섹션에서 자세히 다룰 것입니다. 변수를 가변적으로 만들려면 mut변수 이름 앞에 `variable`을 추가하면 됩니다.

let apples = 5; // immutable
let mut bananas = 5; // mutable
참고: 이 //구문은 줄 끝까지 이어지는 주석을 시작합니다. Rust는 주석 안의 내용을 모두 무시합니다. 주석에 대해서는 3장 에서 더 자세히 다루겠습니다 .

추측 게임 프로그램으로 돌아가서, 이제 여러분은 let mut guess가변 변수 `string`이 생성된다는 것을 알게 되었습니다 guess. 등호(= =)는 Rust에게 이제 `string` 변수에 값을 바인딩하고 싶다는 것을 알려줍니다. 등호 오른쪽에는 guess바인딩된 값이 표시되는데, 이는 `string.getInput()` 함수를 호출한 결과입니다 . `string.getInput( String::new)` 함수는 `string`의 새 인스턴스를 반환합니다 String. String`string`은 표준 라이브러리에서 제공하는 문자열 타입으로, 크기가 조절 가능한 UTF-8 인코딩 텍스트입니다.

::해당 줄의 구문은 이것이 해당 타입의 연관 함수임을 ::new나타냅니다 . 연관 함수 는 타입(이 경우 `string` 타입)에 구현된 함수입니다 . 이 함수는 빈 문자열을 새로 생성합니다. `string` 함수는 새로운 값을 생성하는 함수에 흔히 사용되는 이름이기 때문에 여러 타입에서 찾아볼 수 있습니다 .newStringStringnewnew

전체적으로 보면, 해당 let mut guess = String::new();코드는 현재 새롭고 비어 있는 인스턴스에 바인딩된 변경 가능한 변수를 생성했습니다 String. 휴!

사용자 입력을 받습니다
앞서 프로그램 첫 줄 에 표준 라이브러리의 입출력 기능을 포함시켰던 것을 기억하시죠 ? 이제 모듈 의 함수를 use std::io;호출하여 사용자 입력을 처리해 보겠습니다.stdinio

    io::stdin()
        .read_line(&mut guess)
프로그램 시작 부분에서 해당 io모듈을 임포트하지 않았더라도 , 다음과 같이 함수 호출을 작성하여 해당 함수를 사용할 수 있습니다 . 이 함수는 터미널의 표준 입력에 대한 핸들을 나타내는 타입인 `stdlib` 의 인스턴스를 반환합니다 .use std::io;std::io::stdinstdinstd::io::Stdin

다음으로, 해당 줄은 표준 입력 핸들의 메서드 .read_line(&mut guess)를 호출하여 read_line사용자로부터 입력을 받습니다. 또한, 사용자 입력을 저장할 문자열을 지정하기 위해 &mut guess인수로 문자열을 전달합니다. 이 메서드 의 전체 역할 은 사용자가 표준 입력에 입력한 내용을 덮어쓰지 않고 문자열에 추가하는 것이므로, 해당 문자열을 인수로 전달하는 것입니다. 메서드가 문자열의 내용을 변경할 수 있도록 문자열 인수는 변경 가능해야 합니다.read_lineread_line

`--` 는 &이 인수가 참조임을 나타냅니다 . 참조를 사용하면 코드의 여러 부분에서 하나의 데이터에 접근할 때 데이터를 메모리에 여러 번 복사할 필요가 없습니다. 참조는 복잡한 기능이지만, Rust의 주요 장점 중 하나는 참조를 안전하고 쉽게 사용할 수 있다는 점입니다. 이 프로그램을 완성하는 데에는 이러한 세부 사항을 많이 알 필요는 없습니다. 지금은 변수와 마찬가지로 참조도 기본적으로 불변이라는 점만 알면 됩니다. 따라서 참조를 변경 가능하게 만드는 &mut guess대신 `[[]` 를 사용해야 합니다. (4장에서 참조에 대해 더 자세히 설명합니다.)&guess


잠재적 실패 처리Result
우리는 여전히 이 코드 줄을 작업 중입니다. 지금 세 번째 줄의 텍스트에 대해 논의하고 있지만, 이는 여전히 하나의 논리적인 코드 줄의 일부라는 점에 유의하세요. 다음은 이 메서드입니다.

        .expect("Failed to read line");
이 코드는 다음과 같이 작성할 수도 있습니다.

io::stdin().read_line(&mut guess).expect("Failed to read line");
하지만 한 줄로 길게 작성하면 읽기 어려우므로 나누는 것이 좋습니다. 구문을 사용하여 메서드를 호출할 때 긴 줄을 나누기 위해 줄 바꿈이나 다른 공백을 삽입하는 것이 종종 유용합니다 .method_name(). 이제 이 줄이 무엇을 하는지 살펴보겠습니다.

앞서 언급했듯이, read_line이 함수는 사용자가 입력한 내용을 전달받은 문자열에 넣어주는 동시에 Result값을 반환합니다. 이 함수 Result는 열거 형(enum) 으로 , 여러 가지 가능한 상태 중 하나를 가질 수 있는 타입입니다. 각 가능한 상태를 변형(variant) 이라고 부릅니다 .

6장에서는 열거형에 대해 더 자세히 다룹니다. 이 Result타입의 목적은 오류 처리 정보를 인코딩하는 것입니다.

Result's의 변형은 Ok와 입니다 Err. Ok변형은 작업이 성공했음을 나타내며 성공적으로 생성된 값을 포함합니다. Err변형은 작업이 실패했음을 의미하며 작업이 실패한 이유 또는 방법에 대한 정보를 포함합니다.

다른 모든 타입의 값 Result과 마찬가지로, 해당 타입의 값에도 메서드가 정의되어 있습니다. `Integer` 인스턴스에는 호출할 수 있는 `Integer` 메서드가Result 있습니다 . 이 인스턴스가 `Integer` 값인 경우 , ` Integer` 메서드를 호출하면 프로그램이 종료되고 `Integer` 메서드의 인수로 전달한 메시지가 표시됩니다 . ` Integer` 메서드가 `Available`을 반환 하면 운영 체제에서 발생한 오류일 가능성이 높습니다. 이 인스턴스가 `Integer` 값인 경우 , ` Integer` 메서드는 `Integer` 메서드가 가지고 있는 반환 값을 그대로 반환하여 사용자가 활용할 수 있도록 합니다. 이 경우, 해당 값은 사용자 입력의 바이트 수입니다.expectResultErrexpectexpectread_lineErrResultOkexpectOk

만약 해당 함수를 호출하지 않으면 expect프로그램은 컴파일되지만 경고 메시지가 표시됩니다.

$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
10 |     let _ = io::stdin().read_line(&mut guess);
   |     +++++++

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
ResultRust는 ` return()`에서 반환된 값을 사용하지 않았다고 경고하며 read_line, 이는 프로그램이 발생 가능한 오류를 처리하지 못했음을 나타냅니다.

경고를 억제하는 올바른 방법은 실제로 오류 처리 코드를 작성하는 것이지만, 이 경우에는 문제가 발생했을 때 프로그램을 강제 종료시키기만 하면 되므로 다른 방법을 사용할 수 있습니다 . 오류 복구에 대해서는 9장expect 에서 자세히 알아보겠습니다 .

println!플레이스홀더를 사용한 값 출력
닫는 중괄호를 제외하면, 지금까지 코드에서 논의할 부분은 딱 한 줄뿐입니다.

    println!("You guessed: {guess}");
이 줄은 사용자가 입력한 문자열을 출력합니다. {}중괄호는 자리 표시자입니다. {}마치 게의 집게발처럼 값을 고정하는 역할을 합니다. 변수 값을 출력할 때는 변수 이름을 중괄호 안에 넣을 수 있습니다. 표현식 평가 결과를 출력할 때는 형식 문자열에 빈 중괄호를 넣고, 그 뒤에 쉼표로 구분된 표현식 목록을 각 빈 중괄호 자리 표시자에 같은 순서로 입력합니다. 변수와 표현식 결과를 한 번에 출력하는 코드는 println!다음과 같습니다.

let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
이 코드는 .을 출력합니다 x = 5 and y + 2 = 12.

첫 번째 부분 테스트
추측 게임의 첫 번째 부분을 테스트해 보겠습니다. 다음 명령어를 사용하여 실행하세요 cargo run:

$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
이 시점에서 게임의 첫 번째 부분은 완료되었습니다. 키보드에서 입력을 받아 출력하는 단계입니다.

비밀 번호 생성하기
다음으로, 사용자가 맞춰야 할 비밀번호를 생성해야 합니다. 비밀번호를 매번 다르게 생성해야 게임을 여러 번 플레이해도 재미있습니다. 너무 어렵지 않도록 1에서 100 사이의 난수를 사용하겠습니다. Rust는 아직 표준 라이브러리에 난수 생성 기능을 제공하지 않지만, Rust 팀에서 해당 기능을 제공하는 rand 크레이트를 제공하고 있습니다.


상자를 활용하여 기능성을 향상시키세요
크레이트는 Rust 소스 코드 파일들의 모음이라는 것을 기억하세요. 우리가 지금까지 만들어 온 프로젝트는 실행 가능한 바이너리 크레이트입니다. 이 rand크레이트는 다른 프로그램에서 사용하기 위한 코드를 포함하고 있으며, 자체적으로는 실행될 수 없는 라이브러리 크레이트입니다.

Cargo의 진정한 강점은 외부 크레이트와의 연동에 있습니다. 먼저 , Cargo.toml 파일을 수정하여 해당 크레이트를 종속성으로 추가 rand해야 합니다 . 지금 Cargo.toml 파일을 열고 Cargo가 생성해 준 섹션 헤더 바로 아래에 다음 줄을 추가하세요. 버전 번호를 정확히 입력해야 합니다. 그렇지 않으면 이 튜토리얼의 코드 예제가 제대로 작동하지 않을 수 있습니다.rand[dependencies]rand

파일명: Cargo.toml

[dependencies]
rand = "0.8.5"
Cargo.toml 파일 에서 헤더 다음에 오는 모든 내용은 해당 섹션의 일부이며, 다음 섹션이 시작될 때까지 계속됩니다. `<section>` 섹션에서는 [dependencies]프로젝트가 의존하는 외부 크레이트와 해당 크레이트의 필요한 버전을 Cargo에 알려줍니다. 이 경우, rand시맨틱 버전 지정자 `<script>`를 사용하여 크레이트를 지정합니다 0.8.5. Cargo는 버전 번호 작성 표준인 시맨틱 버전 관리 ( SemVer )를 이해합니다. `<script>` 지정자는 0.8.5실제로 `<script>`의 약어로 ^0.8.5, 0.8.5 이상 0.9.0 미만의 모든 버전을 의미합니다.

Cargo는 이러한 버전들이 0.8.5 버전과 호환되는 공개 API를 가지고 있다고 간주하며, 이 사양을 통해 이 장의 코드와 함께 컴파일되는 최신 패치 릴리스를 받을 수 있습니다. 0.9.0 이상의 버전은 다음 예제에서 사용하는 API와 동일하다고 보장할 수 없습니다.

이제 코드를 전혀 변경하지 않고 목록 2-2에 표시된 대로 프로젝트를 빌드해 보겠습니다.

$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
목록 2-2 : 크레이트를 종속성으로 cargo build추가한 후 실행한 결과rand
운영 체제에 따라 버전 번호가 다를 수 있지만(SemVer 덕분에 모두 코드와 호환됩니다!), 코드 줄 수도 다르고, 줄 순서도 다를 수 있습니다.

외부 종속성을 포함시키면 Cargo는 해당 종속성에 필요한 모든 것의 최신 버전을 레지스트리 에서 가져옵니다. 이 레지스트리는 Crates.io 의 데이터를 복사한 것입니다 . Crates.io는 Rust 생태계 구성원들이 다른 사람들이 사용할 수 있도록 오픈 소스 Rust 프로젝트를 게시하는 곳입니다.

레지스트리를 업데이트한 후, Cargo는 해당 [dependencies]섹션을 확인하고 아직 다운로드되지 않은 크레이트를 모두 다운로드합니다. 이 경우, rand종속성으로만 나열했지만 Cargo는 해당 크레이트가 rand작동하는 데 필요한 다른 크레이트도 함께 다운로드합니다. 크레이트 다운로드가 완료되면 Rust는 이를 컴파일하고, 마지막으로 다운로드된 종속성을 사용하여 프로젝트를 컴파일합니다.

아무런 변경 없이 바로 다시 실행하면 cargo build해당 줄 외에는 아무런 출력도 나타나지 않습니다 Finished. Cargo는 이미 필요한 종속성을 다운로드하고 컴파일했으며, Cargo.toml 파일에서 종속성 관련 설정을 변경하지 않았다는 것을 알고 있기 때문입니다. 또한 Cargo는 코드도 변경하지 않았으므로 다시 컴파일하지 않습니다. 할 일이 없으므로 Cargo는 그냥 종료됩니다.

src/main.rs 파일을 열고 간단한 변경을 한 다음 저장하고 다시 빌드하면 두 줄의 출력만 표시됩니다.

$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
이 줄들은 Cargo가 src/main.rs 파일 에 대한 아주 작은 변경 사항만 반영하여 빌드를 업데이트한다는 것을 보여줍니다 . 종속성은 변경되지 않았으므로 Cargo는 이미 다운로드하고 컴파일한 파일을 재사용할 수 있다는 것을 알고 있습니다.


재현 가능한 빌드 보장
Cargo는 사용자가 코드를 빌드할 때마다 동일한 결과물을 얻을 수 있도록 보장하는 메커니즘을 가지고 있습니다. Cargo는 사용자가 별도로 지정하지 않는 한, 사용자가 지정한 종속성 버전만 사용합니다. 예를 들어, 다음 주에 크레이트 버전 0.8.6이 rand출시되고, 이 버전에 중요한 버그 수정 사항이 포함되어 있지만, 동시에 사용자의 코드를 손상시키는 회귀 오류도 포함되어 있다고 가정해 보겠습니다. 이를 처리하기 위해 Rust는 처음 실행할 때 Cargo.lockcargo build 파일을 생성하며 , 이제 guessing_game 디렉터리에 이 파일이 있습니다.

프로젝트를 처음 빌드할 때 Cargo는 기준에 맞는 모든 종속성 라이브러리의 버전을 파악하여 Cargo.lock 파일에 기록합니다. 이후 프로젝트를 빌드할 때 Cargo는 Cargo.lock 파일이 존재함을 인식하고, 버전을 다시 파악하는 대신 해당 파일에 지정된 버전을 사용합니다. 이를 통해 재현 가능한 빌드를 자동으로 수행할 수 있습니다. 즉, Cargo.lock 파일 덕분에 명시적으로 업그레이드하기 전까지 프로젝트는 0.8.5 버전을 유지합니다. Cargo.lock 파일은 재현 가능한 빌드에 중요하기 때문에 프로젝트의 다른 코드와 함께 소스 코드 관리 시스템에 커밋되는 경우가 많습니다.

크레이트를 업데이트하여 새 버전을 받으세요
크레이트를 업데이트하려면 Cargo에서 제공하는 명령어를 사용하면 됩니다. 이 명령어는 Cargo.lock 파일 을update 무시 하고 Cargo.toml 에 지정된 조건에 맞는 최신 버전을 모두 찾아냅니다 . 그런 다음 Cargo는 해당 버전들을 Cargo.lock 파일 에 기록합니다 . 기본적으로 Cargo는 0.8.5보다 크고 0.9.0보다 작은 버전만 찾습니다. 만약 해당 크레이트에 0.8.6과 0.9.0 두 가지 새 버전이 출시되었다면, 명령어를 실행했을 때 다음과 같은 결과가 표시됩니다 .randcargo update

$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.9.0)
Cargo는 0.9.0 릴리스를 무시합니다. 이 시점에서 Cargo.lock 파일 에도 변경 사항이 표시되어 rand현재 사용 중인 크레이트 버전이 0.8.6임을 알 수 있습니다. rand0.9.0 버전 또는 0.9.x 시리즈의 다른 버전을 사용 하려면 Cargo.toml 파일을 다음과 같이 업데이트해야 합니다 .

[dependencies]
rand = "0.9.0"
다음에 cargo buildCargo를 실행하면 사용 가능한 크레이트 레지스트리가 업데이트되고 rand지정한 새 버전에 따라 요구 사항이 다시 평가됩니다.

Cargo 와 그 생태계에 대해서는 14장에서 더 자세히 다룰 예정이지만, 지금은 이 정도면 충분합니다. Cargo를 사용하면 라이브러리를 재사용하기가 매우 쉬워지므로, Rust 개발자들은 여러 패키지를 조합하여 더 작은 프로젝트를 작성할 수 있습니다.

난수 생성
rand먼저 추측할 숫자를 생성하는 데 사용해 보겠습니다 . 다음 단계는 목록 2-3에 표시된 대로 src/main.rs를 업데이트하는 것입니다.

파일 이름: src/main.rs
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
목록 2-3 : 난수 생성 코드 추가
먼저, 해당 줄을 추가합니다 use rand::Rng;. 이 Rng트레이트는 난수 생성기가 구현해야 하는 메서드를 정의하며, 이러한 메서드를 사용하려면 이 트레이트가 범위 내에 있어야 합니다. 10장에서 트레이트에 대해 자세히 다루겠습니다.

다음으로, 중간에 두 줄을 추가합니다. 첫 번째 줄에서는 rand::thread_rng사용할 특정 난수 생성기를 반환하는 함수를 호출합니다. 이 생성기는 현재 실행 스레드에 로컬로 존재하며 운영 체제에서 시드 값을 가져옵니다. 그런 다음, gen_range 이 난수 생성기의 메서드를 호출합니다. 이 메서드는 앞서 Rng 스코프에 추가한 트레이트 에 정의되어 있습니다 use rand::Rng;. 이 gen_range메서드는 범위 표현식을 인수로 받아 해당 범위 내의 난수를 생성합니다. 여기서 사용하는 범위 표현식은 형식을 취하며 start..=end하한과 상한 모두 포함하므로 1에서 100 사이의 숫자를 요청하려면 으로 지정해야 합니다 1..=100.

참고: 크레이트에서 어떤 트레이트를 사용하고 어떤 메서드와 함수를 호출해야 하는지뿐만 아니라, 각 크레이트에는 사용 방법을 설명하는 문서가 함께 제공됩니다. Cargo의 또 다른 유용한 기능은 명령어를 실행하면 cargo doc --open모든 종속성에서 제공하는 문서를 로컬에 빌드하고 브라우저에서 열어준다는 것입니다. 크레이트의 다른 기능에 관심이 있다면 명령어 rand를 실행 cargo doc --open하고 rand왼쪽 사이드바를 클릭해 보세요.

두 번째 새 줄에는 비밀 숫자가 출력됩니다. 이는 프로그램을 개발하는 동안 테스트할 수 있도록 유용하지만 최종 버전에서는 삭제할 것입니다. 프로그램이 시작하자마자 정답을 출력한다면 게임으로서의 재미가 없어지니까요!

프로그램을 몇 번 실행해 보세요.

$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
여러분은 서로 다른 난수를 얻었을 것이고, 그 숫자들은 모두 1에서 100 사이의 숫자일 것입니다. 아주 잘했어요!

추측한 숫자와 비밀 숫자를 비교하기
이제 사용자 입력과 난수가 있으므로 둘을 비교할 수 있습니다. 해당 단계는 목록 2-4에 나와 있습니다. 참고로, 이 코드는 아직 컴파일되지 않습니다. 그 이유는 나중에 설명하겠습니다.

파일 이름: src/main.rs
이 코드는 컴파일되지 않습니다!
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
목록 2-4 : 두 숫자를 비교할 때 발생할 수 있는 반환 값 처리
먼저, 표준 라이브러리에서 use가져온 타입을 범위 내로 불러오는 문장을 추가합니다 . 이 타입은 또 다른 열거형이며 , , , 의 변형을 가집니다 . 이 세 가지는 두 값을 비교할 때 가능한 결과입니다.std::cmp::OrderingOrderingLessGreaterEqual

다음으로, 해당 타입을 사용하는 다섯 줄의 새 코드를 맨 아래에 추가합니다 Ordering. 이 cmp메서드는 두 값을 비교하며 비교 가능한 모든 객체에 대해 호출할 수 있습니다. 비교 대상에 대한 참조를 인수로 받습니다. 여기서는 guess와 비교합니다. 그런 다음, 문을 사용하여 범위 내로 가져온 열거형 secret_number의 변형을 반환합니다 . 와 의 값을 사용하여 호출했을 때 어떤 변형이 반환되었는지에 따라 다음에 수행할 작업을 결정하는 표현식을 사용합니다 .OrderingusematchOrderingcmpguesssecret_number

표현식 은 암(arm)들match 로 구성됩니다 . 각 암은 비교 대상 패턴 과, 주어진 값이 해당 암의 패턴 에 부합할 경우 실행될 코드 로 이루어져 있습니다. Rust는 주어진 값을 받아 각 암의 패턴을 차례로 검사합니다. 패턴과 `with` 구문은 Rust의 강력한 기능으로, 코드가 마주칠 수 있는 다양한 상황을 표현하고 모든 상황을 처리할 수 있도록 해줍니다. 이 기능들은 각각 6장과 19장에서 자세히 다룰 예정입니다.matchmatchmatch

여기서 사용하는 표현을 예시를 통해 살펴보겠습니다 match. 사용자가 50을 추측했고 이번에 무작위로 생성된 비밀 숫자가 38이라고 가정해 보겠습니다.

코드가 50과 38을 비교하면 50이 38보다 크기 때문에 cmp메서드는 반환됩니다 . 표현식은 값을 가져와 각 분기의 패턴을 확인하기 시작합니다. 첫 번째 분기의 패턴인 를 살펴보고 값이 일치하지 않으므로 해당 분기의 코드를 무시하고 다음 분기로 넘어갑니다. 다음 분기의 패턴은 이고 , 이는 와 일치 합니다 ! 해당 분기의 관련 코드가 실행되어 화면에 출력됩니다 . 표현식은 첫 번째 일치 후 종료되므로 이 시나리오에서는 마지막 분기를 확인하지 않습니다.Ordering::GreatermatchOrdering::GreaterOrdering::LessOrdering::GreaterOrdering::LessOrdering::GreaterOrdering::GreaterToo big!match

하지만 목록 2-4의 코드는 아직 컴파일되지 않습니다. 한번 시도해 보겠습니다.

$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
   |                 |
   |                 arguments to this method are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: method defined here
  --> /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library/core/src/cmp.rs:964:8

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error
오류의 핵심은 타입이 일치하지 않는다는 것입니다 . Rust는 강력한 정적 타입 시스템을 가지고 있지만, 타입 추론 기능도 제공합니다. 예를 들어 `string`을 작성했을 때 Rust는 `string`이 문자열 이어야 한다고 let mut guess = String::new()추론하여 타입을 명시적으로 지정할 필요가 없도록 했습니다. 반면 `string`은 숫자 타입입니다. Rust의 숫자 타입 중에는 1부터 100까지의 값을 가질 수 있는 타입이 몇 가지 있습니다 . 예를 들어 32비트 숫자 `string` , 부호 없는 32비트 숫자 `string` , 64비트 숫자 `string` 등이 있습니다. 특별히 지정하지 않는 한, Rust는 기본적으로 `string`을 사용하는데 , 이는 `string`의 타입입니다 . Rust가 다른 숫자 타입을 추론하도록 타입 정보를 추가하지 않는 한 마찬가지입니다. 이 오류는 Rust가 문자열과 숫자 타입을 직접 비교할 수 없기 때문에 발생합니다.guessStringsecret_numberi32u32i64i32secret_number

궁극적으로 우리는 프로그램이 입력으로 읽은 값을 숫자형으로 변환하여 String비밀 숫자와 수치적으로 비교하고자 합니다. 이를 위해 main함수 본문에 다음 줄을 추가합니다.

파일 이름: src/main.rs

    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
그 대사는 다음과 같습니다:

let guess: u32 = guess.trim().parse().expect("Please type a number!");
우리는 라는 변수를 생성합니다 guess. 그런데 잠깐, 프로그램에 이미 라는 변수가 있지 않나요 guess? 맞습니다. 하지만 Rust에서는 기존 변수 값을 guess새로운 변수로 덮어쓰기(shadowing)하는 기능을 사용할 수 있습니다. 이 기능을 사용guess 하면 두 개의 고유한 변수를 만들 필요 없이, 예를 들어 guess_str와 같이 변수 이름을 재사용할 수 있습니다 . 이 기능은 3장guess 에서 더 자세히 다루겠지만 , 지금은 이 기능이 한 데이터 타입을 다른 데이터 타입으로 변환할 때 자주 사용된다는 것을 알아두세요.

이 새로운 변수를 표현식에 바인딩합니다 guess.trim().parse(). 표현식의 는 입력값을 문자열로 담고 있던 guess 원래 변수를 가리킵니다 . 인스턴스 의 메서드 는 문자열 의 시작과 끝에 있는 공백을 제거합니다. 이는 문자열을 숫자 데이터만 담을 수 있는 숫자로 변환하기 전에 반드시 수행해야 하는 작업입니다. 사용자는 조건을 만족시키기 위해 를 누르고 추측값을 입력해야 하는데, 이 추측값은 문자열에 줄 바꿈 문자를 추가합니다. 예를 들어, 사용자가 를 입력 하고 를 누르면 다음 과 같이 표시됩니다 . 는 "줄 바꿈"을 나타냅니다. (Windows에서는 를 누르면 캐리지 리턴과 줄 바꿈 문자 가 입력 됩니다 .) 메서드는 또는 를 제거하여 최종적으로 만 남깁니다 .guesstrimStringu32enterread_line5enterguess5\n\nenter\r\ntrim\n\r\n5

parse문자열의 `convert` 메서드 는 문자열을 다른 유형으로 변환합니다. 여기서는 문자열을 숫자로 변환하는 데 사용합니다. Rust에게 원하는 정확한 숫자 유형을 알려주기 위해 `number` 타입 어노테이션을 사용합니다 let guess: u32. 콜론( ::) guess은 변수의 유형을 어노테이션으로 지정할 것임을 Rust에 알려줍니다. Rust에는 몇 가지 내장 숫자 유형이 있습니다. 여기서 `number`는 부호 없는 32비트 정수입니다. 작은 양수를 표현할 때 좋은 기본 선택입니다. 다른 숫자 유형에 대해서는 3장u32 에서 자세히 알아보겠습니다 .

또한, u32이 예제 프로그램의 어노테이션과 비교 조건 덕분에 Rust는 도 동일한 타입이어야 한다고 secret_number추론합니다 . 따라서 이제 비교는 같은 타입의 두 값 사이에서 이루어집니다!secret_numberu32

이 parse메서드는 논리적으로 숫자로 변환할 수 있는 문자에만 적용되므로 오류가 발생하기 쉽습니다. 예를 들어 문자열에 빈 문자열("")이 포함되어 있다면 A👍%이를 숫자로 변환할 방법이 없습니다. 따라서 이 메서드는 실패할 가능성이 있기 때문에 앞서 "실행 오류 처리 " 에서 설명한 메서드 와 마찬가지로 타입을 parse반환합니다 . 이 경우에도 동일한 방식으로 메서드를 다시 사용해 보겠습니다 . 메서드가 문자열에서 숫자를 생성할 수 없어서 예외를 반환하면 게임이 종료되고 지정된 메시지가 출력됩니다. 메서드가 문자열을 숫자로 성공적으로 변환하면 예외의 타입을 반환하고 , 메서드는 해당 값에서 원하는 숫자를 반환합니다 .Resultread_lineResultResultexpectparseErr ResultexpectparseOkResultexpectOk

이제 프로그램을 실행해 보겠습니다.

$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
훌륭합니다! 추측 값 앞에 공백이 추가되었음에도 불구하고 프로그램은 사용자가 76을 추측했다는 것을 정확하게 알아냈습니다. 다양한 입력값(정답, 너무 큰 숫자, 너무 작은 숫자)에 따라 프로그램이 어떻게 다르게 동작하는지 확인하기 위해 프로그램을 몇 번 실행해 보세요.

이제 게임의 대부분은 작동하지만, 사용자는 한 번만 추측할 수 있습니다. 반복문을 추가하여 이 문제를 해결해 봅시다!

반복문을 사용하여 여러 번 추측할 수 있도록 허용
해당 loop키워드는 무한 루프를 생성합니다. 사용자가 숫자를 맞출 수 있는 기회를 더 많이 주기 위해 루프를 추가하겠습니다.

파일 이름: src/main.rs

    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
보시다시피, 추측 입력 프롬프트부터 모든 코드를 반복문 안으로 옮겼습니다. 반복문 안의 줄들을 각각 네 칸씩 들여쓰기하고 프로그램을 다시 실행해 보세요. 이제 프로그램은 계속해서 추측을 입력하도록 요구할 텐데, 여기서 새로운 문제가 발생합니다. 사용자가 프로그램을 종료할 수 없는 것처럼 보입니다!

사용자는 키보드 단축키 ctrl- 를 사용하여 언제든지 프로그램을 중단할 수 있습니다. 하지만 "추측과 비밀 숫자 비교" 에서 C언급했듯이, 이 끝없는 괴물로부터 벗어나는 또 다른 방법이 있습니다. 사용자가 숫자가 아닌 답을 입력하면 프로그램이 종료됩니다. 우리는 이 점을 이용하여 사용자가 프로그램을 종료할 수 있도록 만들 수 있습니다. 그 예는 다음과 같습니다.parse

$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
타이핑을 하면 quit게임이 종료되지만, 보시다시피 숫자 이외의 다른 입력을 해도 게임이 종료됩니다. 이는 최소한 최적의 방식은 아닙니다. 정답을 맞혔을 때도 게임이 종료되도록 하고 싶습니다.

정답을 맞춘 후 포기하기
사용자가 게임에서 이겼을 때 게임이 종료되도록 다음 break과 같은 구문을 추가하여 프로그램을 작성해 보겠습니다.

파일 이름: src/main.rs

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
그 break줄을 추가하면 You win!사용자가 비밀번호를 맞혔을 때 프로그램이 루프를 종료하게 됩니다. 루프가 프로그램의 마지막 부분이기 때문에 루프를 종료한다는 것은 프로그램을 완전히 종료하는 것을 의미합니다 main.

잘못된 입력 처리
게임의 동작을 더욱 개선하기 위해, 사용자가 숫자가 아닌 값을 입력했을 때 프로그램이 종료되는 대신, 숫자가 아닌 값을 무시하여 사용자가 계속해서 추측할 수 있도록 해 보겠습니다. 이를 위해 목록 2-5에 나와 있는 것처럼, 를 로 guess변환하는 줄을 수정하면 됩니다 .Stringu32

파일 이름: src/main.rs
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // --snip--
목록 2-5 : 숫자가 아닌 값을 입력했을 때 프로그램을 종료하는 대신 무시하고 다른 값을 입력하도록 요청하는 방법
오류 발생 시 크래시를 방지하고 오류를 처리하기 위해 표현식 expect호출 로 전환합니다 . 반환 값은 타입이며, 변형 값 들을 가진 열거형이라는 점 을 기억하세요. 메서드 결과 에서와 마찬가지로 여기에서도 표현식을 사용합니다 .matchparseResultResultOkErrmatchOrderingcmp

문자열을 숫자로 성공적으로 변환 하면 , 변환된 숫자를 포함하는 값을 parse반환합니다 . 이 값은 첫 번째 함수의 패턴과 일치하며, 표현식은 생성된 값을 그대로 반환하여 해당 값 안에 저장합니다 . 이렇게 생성된 숫자는 우리가 새로 생성하는 변수의 원하는 위치에 저장됩니다 .OkOkmatchnumparseOkguess

문자열을 숫자로 변환하는 데 실패parse 하면 오류 에 대한 자세한 정보를 담은 값을 반환합니다 . 이 값은 첫 번째 단계의 패턴과 는 일치하지 않지만 두 번째 단계의 패턴 과는 일치합니다 . 밑줄( _)은 모든 값을 허용하는 값입니다. 이 예에서는 값에 어떤 정보가 포함되어 있든 상관없이 모든 값과 일치하도록 지정합니다 . 따라서 프로그램은 두 번째 단계의 코드를 실행하여 다음 반복으로 이동 하고 다른 추측을 요청합니다. 결과적으로 프로그램은 발생할 수 있는 모든 오류를 무시하게 됩니다!ErrErrOk(num)matchErr(_)_Errcontinueloopparse

이제 프로그램의 모든 기능이 예상대로 작동해야 합니다. 한번 시도해 봅시다.

$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
멋지네요! 마지막으로 아주 작은 수정만 하면 추측 게임을 끝낼 수 있습니다. 프로그램이 여전히 비밀 숫자를 출력하고 있다는 것을 기억하세요. 테스트할 때는 괜찮았지만, 게임의 재미를 떨어뜨립니다. println!비밀 숫자를 출력하는 부분을 삭제해 보겠습니다. 목록 2-6은 최종 코드입니다.

파일 이름: src/main.rs
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
목록 2-6 : 추측 게임 전체 코드
이제 여러분은 추측 게임을 성공적으로 만들었습니다. 축하합니다!

요약
let이 프로젝트는 Rust의 여러 새로운 개념, 즉 변수, 데이터 타입, 함수, 외부 크레이트 사용법 등 을 직접 체험하며 배우는 기회를 제공합니다 match. 다음 몇 장에 걸쳐 이러한 개념들을 더 자세히 살펴보겠습니다. 3장에서는 변수, 데이터 타입, 함수와 같이 대부분의 프로그래밍 언어에서 공통적으로 다루는 개념들을 Rust에서 사용하는 방법을 보여줍니다. 4장에서는 Rust를 다른 언어와 차별화하는 특징인 소유권에 대해 알아봅니다. 5장에서는 구조체와 메서드 구문을, 6장에서는 열거형의 작동 방식을 설명합니다.