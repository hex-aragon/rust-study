안녕하세요, Cargo 담당자님!

Cargo는 Rust의 빌드 시스템이자 패키지 관리자입니다. 대부분의 Rust 개발자는 이 도구를 사용하여 Rust 프로젝트를 관리하는데, Cargo가 코드 빌드, 코드에 필요한 라이브러리 다운로드 및 해당 라이브러리 빌드와 같은 많은 작업을 자동으로 처리해 주기 때문입니다. (코드에 필요한 라이브러리를 종속성 이라고 합니다 .)

지금까지 작성한 것과 같은 가장 간단한 Rust 프로그램은 의존성이 없습니다. 만약 "Hello, world!" 프로젝트를 Cargo로 빌드했다면, Cargo의 빌드 관련 기능만 사용했을 것입니다. 하지만 더 복잡한 Rust 프로그램을 작성하게 되면 의존성이 추가될 것이고, Cargo를 사용하여 프로젝트를 시작하면 의존성을 추가하는 것이 훨씬 쉬워집니다.

대부분의 Rust 프로젝트에서 Cargo를 사용하기 때문에 이 책의 나머지 부분에서는 여러분도 Cargo를 사용하고 있다고 가정합니다. "설치" 섹션에서 설명한 공식 설치 프로그램을 사용했다면 Cargo는 Rust와 함께 설치됩니다. 다른 방법으로 Rust를 설치했다면 터미널에서 다음 명령어를 입력하여 Cargo가 설치되었는지 확인하십시오.

$ cargo --version
버전 번호가 보이면 해당 버전이 설치된 것입니다! 오류 메시지(예: )가 표시되면 command not found설치 방법에 대한 설명서를 참조하여 Cargo를 별도로 설치하는 방법을 확인하십시오.

Cargo를 사용하여 프로젝트 생성
Cargo를 사용하여 새 프로젝트를 만들고 이전의 "Hello, world!" 프로젝트와 어떻게 다른지 살펴보겠습니다. 프로젝트 디렉토리(또는 코드를 저장하기로 결정한 위치)로 이동합니다. 그런 다음, 모든 운영 체제에서 다음 명령을 실행합니다.

$ cargo new hello_cargo
$ cd hello_cargo
첫 번째 명령은 hello_cargo 라는 새 디렉터리와 프로젝트를 생성합니다 . 프로젝트 이름을 hello_cargo 로 지정했으므로 Cargo는 동일한 이름의 디렉터리에 파일을 생성합니다.

hello_cargo 디렉토리 로 이동하여 파일 목록을 확인하세요. Cargo가 Cargo.toml 파일과 main.rs 파일이 있는 src 디렉토리, 이렇게 두 개의 파일과 하나의 디렉토리를 생성했음을 알 수 있습니다 .

또한 새 Git 저장소와 .gitignorecargo new 파일을 생성했습니다. 기존 Git 저장소 내에서 실행하면 Git 파일이 생성되지 않습니다 . .gitignore 파일을 사용하여 이 동작을 재정의할 수 있습니다 cargo new --vcs=git.

참고: Git은 일반적인 버전 관리 시스템입니다. 플래그 cargo new를 사용하면 다른 버전 관리 시스템을 사용하거나 버전 관리 시스템을 사용하지 않도록 변경할 수 있습니다 --vcs. cargo new --help사용 가능한 옵션을 보려면 명령어를 실행하세요.

원하는 텍스트 편집기로 Cargo.toml 파일을 엽니다 . 파일 내용은 목록 1-2의 코드와 유사해야 합니다.

파일명: Cargo.toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
목록 1-2 : Cargo.toml 파일 의 내용 ( 생성됨)cargo new
이 파일은 Cargo의 설정 형식인 TOML ( Tom's Obvious, Minimal Language ) 형식입니다.

첫 번째 줄인 [package]는 다음 문장들이 패키지를 구성하는 내용임을 나타내는 섹션 제목입니다. 이 파일에 더 많은 정보를 추가하면 다른 섹션도 추가할 것입니다.

다음 세 줄은 Cargo가 프로그램을 컴파일하는 데 필요한 구성 정보(이름, 버전, 사용할 Rust 에디션)를 설정합니다. 키에 대해서는 부록 Eedition 에서 설명하겠습니다 .

마지막 줄은 [dependencies]프로젝트의 의존성 목록을 작성하는 섹션의 시작 부분입니다. Rust에서는 코드 패키지를 크레이트(crate) 라고 합니다 . 이 프로젝트에서는 다른 크레이트가 필요하지 않지만, 2장의 첫 번째 프로젝트에서는 필요하므로 이 의존성 섹션을 그때 사용하겠습니다.

이제 src/main.rs 파일을 열고 내용을 살펴보세요.

파일 이름: src/main.rs

fn main() {
    println!("Hello, world!");
}
Cargo가 목록 1-1에서 우리가 작성한 것과 똑같은 "Hello, world!" 프로그램을 생성해 주었습니다! 지금까지 우리 프로젝트와 Cargo가 생성한 프로젝트의 차이점은 Cargo가 코드를 src 디렉토리에 배치했고, 우리는 최상위 디렉토리에 Cargo.toml 설정 파일을 가지고 있다는 점입니다 .

Cargo는 소스 파일이 src 디렉토리 안에 있어야 한다고 가정합니다. 최상위 프로젝트 디렉토리는 README 파일, 라이선스 정보, 설정 파일 등 코드와 관련 없는 모든 파일을 위한 공간입니다. Cargo를 사용하면 프로젝트를 체계적으로 관리할 수 있습니다. 모든 파일에는 제자리가 있고, 모든 파일은 제자리에 있습니다.

"Hello, world!" 프로젝트처럼 Cargo를 사용하지 않는 프로젝트를 시작했다면, Cargo를 사용하는 프로젝트로 변환할 수 있습니다. 프로젝트 코드를 src 디렉토리로 이동하고 적절한 Cargo.toml 파일을 생성하세요. Cargo.toml 파일을 쉽게 얻는 방법 중 하나는 ` cargo.toml` 명령어 를 실행하는 것입니다 cargo init. 이 명령어가 자동으로 파일을 생성해 줍니다.

화물 운송 프로젝트 구축 및 운영
이제 Cargo를 사용하여 "Hello, world!" 프로그램을 빌드하고 실행할 때 어떤 점이 다른지 살펴보겠습니다! hello_cargo 디렉토리에서 다음 명령어를 입력하여 프로젝트를 빌드하세요.

$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
이 명령은 현재 디렉터리가 아닌 target/debug/hello_cargo ( Windows에서는 target\debug\hello_cargo.exe ) 에 실행 파일을 생성합니다 . 기본 빌드는 디버그 빌드이므로 Cargo는 바이너리를 debug 라는 디렉터리에 저장합니다 . 다음 명령으로 실행 파일을 실행할 수 있습니다.

$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
모든 것이 순조롭게 진행되면 Hello, world!터미널에 출력될 것입니다. cargo build처음 실행 시 Cargo는 최상위 디렉터리에 Cargo.lock 이라는 새 파일을 생성합니다 . 이 파일은 프로젝트에 필요한 종속성의 정확한 버전을 관리합니다. 이 프로젝트에는 종속성이 없으므로 파일 내용이 다소 간략합니다. Cargo가 파일 내용을 관리해 주기 때문에 이 파일을 수동으로 수정할 필요는 없습니다.

우리는 방금 를 사용하여 프로젝트를 빌드 cargo build하고 실행했지만 ./target/debug/hello_cargo, 를 사용하여 cargo run코드를 컴파일한 다음 결과 실행 파일을 실행하는 모든 작업을 한 번의 명령으로 수행할 수도 있습니다.

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
cargo run명령어를 실행하는 것을 기억하고 cargo build실행 파일의 전체 경로를 입력하는 것보다 `.` 을 사용하는 것이 더 편리하기 때문에 대부분의 개발자는 `.`을 사용합니다 cargo run.

이번에는 Cargo가 컴파일 중이라는 출력이 표시되지 않았다는 점에 주목하세요 hello_cargo. Cargo는 파일이 변경되지 않았다고 판단하여 다시 빌드하지 않고 바로 바이너리를 실행했습니다. 만약 소스 코드를 수정했다면 Cargo는 실행 전에 프로젝트를 다시 빌드했을 것이고, 다음과 같은 출력이 표시되었을 것입니다.

$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
Cargo는 또한 라는 명령어를 제공합니다 cargo check. 이 명령어는 코드가 컴파일은 되지만 실행 파일이 생성되지 않는지 빠르게 확인합니다.

$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
실행 파일을 사용하지 않을 이유가 있을까요? `git add`는 실행 파일 생성 단계를 건너뛰기 때문에 `git add` cargo check보다 훨씬 빠른 경우가 많습니다. 코드를 작성하는 동안 지속적으로 컴파일 여부를 확인하는 경우, `git add`를 사용하면 프로젝트 컴파일 상태를 더 빠르게 확인할 수 있습니다! 따라서 많은 Rust 개발자들은 프로그램을 작성하는 동안 주기적으로 `git add`를 실행하여 컴파일이 제대로 되는지 확인합니다. 그리고 실행 파일을 사용할 준비가 되면 `git add`를 실행합니다 .cargo buildcargo checkcargo checkcargo build

지금까지 Cargo에 대해 배운 내용을 정리해 보겠습니다.

우리는 를 사용하여 프로젝트를 생성할 수 있습니다 cargo new.
우리는 를 사용하여 프로젝트를 구축할 수 있습니다 cargo build.
우리는 를 사용하여 한 단계로 프로젝트를 빌드하고 실행할 수 있습니다 cargo run.
바이너리 파일을 생성하지 않고도 프로젝트를 빌드하여 오류를 확인할 수 있습니다 cargo check.
Cargo는 빌드 결과를 코드와 같은 디렉토리에 저장하는 대신 target/debug 디렉토리에 저장합니다.
Cargo를 사용하는 또 다른 장점은 어떤 운영 체제를 사용하든 명령어가 동일하다는 것입니다. 따라서 이 시점부터는 Linux 및 macOS와 Windows에 대한 구체적인 사용 설명은 제공하지 않겠습니다.

릴리스용 빌드 중
프로젝트가 최종적으로 릴리스 준비가 되면, `compilation` cargo build --release명령어를 사용하여 최적화를 적용하여 컴파일할 수 있습니다. 이 명령어를 실행하면 `target/debug` 대신 `target/release` 폴더 에 실행 파일이 생성됩니다 . 최적화를 통해 Rust 코드의 실행 속도가 향상되지만, 최적화를 활성화하면 컴파일 시간이 길어집니다. 따라서 두 가지 프로파일이 있습니다. 하나는 개발 환경에서 빠르게 자주 재빌드할 때 사용하는 프로파일이고, 다른 하나는 최종 배포용 프로그램을 빌드하여 반복적인 재빌드 없이 최대한 빠르게 실행할 때 사용하는 프로파일입니다. 코드 실행 시간을 벤치마킹할 때는 ` target/release` 폴더에 있는 실행 파일을 사용하여 벤치마킹하는 것이 좋습니다 .cargo build --release


Cargo의 관례 활용
간단한 프로젝트의 경우 Cargo는 일반적인 빌드 도구보다 큰 이점을 제공하지 않지만 rustc, 프로그램이 복잡해질수록 그 진가가 드러납니다. 프로그램이 여러 파일로 구성되거나 특정 라이브러리에 대한 의존성이 필요하게 되면 Cargo를 통해 빌드를 관리하는 것이 훨씬 간편해집니다.

이 프로젝트는 간단하지만 hello_cargo, 앞으로 Rust를 사용하면서 실제로 활용하게 될 많은 도구를 포함하고 있습니다. 기존 프로젝트를 작업하려면 다음 명령어를 사용하여 Git으로 코드를 체크아웃하고, 해당 프로젝트 디렉토리로 이동한 다음 빌드할 수 있습니다.

$ git clone example.org/someproject
$ cd someproject
$ cargo build
Cargo에 대한 자세한 내용은 관련 문서를 참조하십시오 .

