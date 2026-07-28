use clap::Parser;
#[derive(Parser)]
#[clap(version = "1.0")] //추가 
struct Args {
    arg1: String, 
    arg2: String,
}

fn main() {
    //명령줄 인수 목록 출력하기
    // for arg in std::env::args() {
    //     println!("{}", arg);
    // }

    //구조체 Args로 정의한 타입 인수를 받아서 parse 실행 
    let _args = Args::parse();
}
