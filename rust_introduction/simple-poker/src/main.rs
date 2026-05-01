
use rand::seq::SliceRandom; 

//열거형 타입
#[derive(Copy, Clone, PartialEq, Debug)] 
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Copy, Clone, PartialEq,Debug)] 
//구조체
struct Card {
    //위에서 정의한 열거 타입 사용 
    suit: Suit,
    rank: i32,
}


fn main() {
    
    // let suit = Suit::Club;
    // let rank = 1;
    // let card = Card {suit, rank};
    // println!("{:?}",card);


    //Vec 준비 
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    //카드 더미 작성 
    for suit in suits {
        for rank in 1..=13 {
            //vec 에 카드 추가
            deck.push(Card {suit, rank});
        }
    }

    //카드 더미 섞기
    let mut rng = rand::rng(); //추가
    deck.shuffle(&mut rng); //추가 
    println!("{:?}", deck);

    // 패용 Vec 준비
    let mut hand: Vec<Card> = Vec::new();
    // 카드 5장 뽑기
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    //패 표시
    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    //패 정렬
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    //패 표시
    println!("---hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    println!("교환하고 싶은 카드 번호를 입력하세요 (예 : 1 2 3 )");
    //사용자에게 받은 입력을 저장할 변수
    let mut input = String::new();
    //사용자가 입력한 내용을 변수에 저장
    std::io::stdin().read_line(&mut input).unwrap();


    //다루기 쉽게 Vec로 변환
    let numbers: Vec<usize> = input
        .split_whitespace() //문자열을 스페이스 단위로 분할 (예 : "1 2 3" -> ["1", "2", "3"])
        .map(|x| x.parse().unwrap()) //문자열을 숫자로 변환 (예 : ["1", "2", "3"] -> [1, 2, 3])
        .collect::<Vec<usize>>(); //Vec로 변환

    //지정한 위치의 카드를 카드 더미에서 꺼낸 카드로 교환 
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    //패 정렬
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    //패 표시
    println!("---Hand---");
    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }


    //플러시 확인
    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);
    //페어 확인
    let mut count = 0;

    for i in 0..hand.len() -1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("플러시!");
    } else if count >= 3 {
        println!("쓰리 카드!");
    } else if count == 2 {
        println!("투 페어!");
    } else if count == 1 {
        println!("원 페어!");
    } else {
        println!("노 페어..");
    }
        
}
