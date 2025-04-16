//Other collection types
//HashMap, BTreeMap
//Key, Value
//Key : String
//Value: Vec<String>
//land: 나라, 국가 
//HashMap<String, Vec<String> >> 순서 X 
//BTreeMap  순서 O ? 
//use std::collections::HashMap;
use std::collections::BTreeMap;

struct City {
    name: String,
   // population: HashMap<u32, u32> //year + population
    population: BTreeMap<u32, u32> //year + population
}

fn main() {
    
    let mut tallinn = City {
        name: "Tallin".to_string(),
       // population: HashMap::new()
        population: BTreeMap::new()
    };
    
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);
    
    //해시맵은 랜덤하게 출력된다. 
    //ordering 할 수 없는 게 특징 
    for (year, population) in tallinn.population {
        println!("In the year {} the population was {}", year, population);
    }
    
}
