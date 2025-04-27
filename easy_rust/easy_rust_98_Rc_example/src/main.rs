use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: String, 
    population: u32,
    history: Rc<String> 
}

#[derive(Debug)]
struct CityData {
    names : Vec<String>,
    histories: Vec<Rc<String>>
}

fn main() {

    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_336_000,
        history: Rc::new("Calgary was founded in blah blah blah".to_string())
    };

    let canada_cities = CityData {
        names: vec![calgary.name],
        histories: vec![Rc::clone(&calgary.history)]
    };

    println!("Calgary's history is : {}", calgary.history);

    println!("Data has {} owners", Rc::strong_count(&calgary.history));

    // weak_count 
    // Rc drop     
}
