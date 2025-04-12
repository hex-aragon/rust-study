// struct = and
// enum = or 

//enumeration 
// e = from 
// number


enum ThingsInTheSky {
    Sun, //0 
    Stars //1 
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars
    }
}

fn check_skystate(state : &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("I can see the stars")
        
    }
}

fn main() {
    let time = 8;
    let sky_state = create_skystate(time);
    check_skystate(&sky_state);
    
    let time2 = 20;
    let sky_state2 = create_skystate(time2);
    check_skystate(&sky_state2);
    
    check_skystate(&create_skystate(6));
}
