enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry
}

enum Season {
    Spring, //0
    Summer, //1
    Autumn, //2
    Winter 
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;

    // let happiness_level = match mood {
    //     Mood::Happy => 10,
    //     Mood::Sleepy => 6,
    //     Mood::NotBad => 7,
    //     Mood::Angry => 2
    // };
   
   //use Mood::*를 쓰면서 위와 같이 쓰던것을 아래와 같이 쓸 수 있다.  
    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2
    }
    //; 빼면 자동 리턴 ?
    
    
   // happiness_level 이 변수도 설정할 필요 없음 
}

fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);
    
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter]; //Vec<Season>
    
    for season in four_seasons {
        println!("The number is {}", season as u32);
    }
    
}
