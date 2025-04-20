use std::fmt::Display;

fn print_vec<T: Display>(input: &Vec<T>){
    for item in input {
        print!("{item}");
    }
    println!();
}


#[derive(Debug, Clone)]
struct City {
    name: String,
    population: u32, 
}

#[derive(Debug, Clone)] //Country also needs to be printed
struct Country { 
    cities: Vec<City>, // Our cities go in here 
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population
        }
    }
}

//Country::from(vec![City, City])
impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self {
            cities
        }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}", city.name, city.population);
        }
    }
}

fn main() {

    let array_vec = Vec::from([8,9,10]);
    print_vec(&array_vec);

    let str_vec = Vec::from("What kind of vec is this?");
    print_vec(&str_vec);

    let string_vec = Vec::from("What kind of Vec is a String vec?");
    print_vec(&string_vec);

    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);
    let finland_cities = vec![helsinki, turku];
    //let finland = Country::from(finland_cities);
    let finland = Country::from(finland_cities.clone());
    finland.print_cities();

    println!("");
    let finland2: Country = finland_cities.into();
    finland2.print_cities();
    //.into() 사용 예시 ? 
    //let x = ...iter().for_each().into()

}
