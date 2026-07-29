use clap::{Args, Parser, Subcommand};
// csv 크레이트의 Writer(파일 쓰기 모듈) 사용 선언
use csv::Writer;

#[derive(Parser)]
#[clap(version = "1.0")] //추가 
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 신규 계좌 작성
    New,
    /// 계좌 입금
    Deposit,
    /// 계좌 출금
    Withdraw,
    /// CSV에서 가져오기
    Import,
    /// 리포트 작성
    Report,
}

fn main() {
    //명령줄 인수 목록 출력하기
    // for arg in std::env::args() {
    //     println!("{}", arg);
    // }

    //구조체 Args로 정의한 타입 인수를 받아서 parse 실행 
    let args = Args::parse();

    match args.command {
        Command::New => new(),
        Command::Deposit => unimplemented!(),
        Command::Withdraw => unimplemented!(),
        Command::Import => unimplemented!(),
        Command::Report => unimplemented!(),
    }
}

// TODO: 구현하기
fn new() {
    println!("New command");
    // accounts.csv로 CSV 파일 작성
    let mut writer = Writer::from_path("accounts.csv").unwrap();
    writer.write_record(["날짜", "용도", "금액"]).unwrap(); //헤더 쓰기
    writer.flush().unwrap()
}

