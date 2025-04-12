// struct == and
// enum = or 

// unit struct
struct FileDirectory;

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}

// 새로운 타입에 trait가 없어서 디버그 프린트 진행해야함 
//tuple struct 
#[derive(Debug)] //attribute 
struct Colour(u8, u8, u8);

// named struct 
#[derive(Debug)]
struct Country {
    population: u32,
    capital: String, 
    leader_name: String
}

fn main() {

    let x  = FileDirectory;
    // takes_file_directory(x);
    // println!("Hello World!");
    
    println!("The size is {}",    std::mem::size_of_val(&x));
    
    let my_colour = Colour(20, 50, 100);
    println!("The first colour is {}", my_colour.0);
    println!("The second colour is {}", my_colour.1);
    println!(" colour is {:?}", my_colour);
    
    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string()
    };
    
    println!("The country is : {:?}", canada);
    println!("The canada population is: {}, canada capital is {} ", canada.population, canada.capital);
    
    
}
