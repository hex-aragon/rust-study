use std::mem::size_of_val;

struct Numbers {
    one: u8,
    two: u8,
    three: u8,
    four: u32
}

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String, 
    leader_name: String,
    all_populations: [u32; 5500]
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justing Trudeau".to_string();
    
    // let my_country = Country {
    //     population: population,
    //     capital: capital,
    //     leader_name: leader_name
    // };
      let my_country = Country {
        population,
        capital,
        leader_name,
        all_populations:[500; 5500]
    };
    
    //println!("{:#?}",my_country);
    
    println!("Country is {} bytes in size", size_of_val(&my_country));
    
    let numbers = Numbers {
        one : 8,
        two: 19,
        three: 20,
        four: 30
    };
    
    println!("Size is : {}", size_of_val(&numbers));
    
    
    
    
    
}
