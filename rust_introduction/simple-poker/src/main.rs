
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
}
