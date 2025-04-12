fn main() {
    
    let sky = "cloudy"; // &str 
    let temperature = "warm";
    
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _) => println!("Cloudy and something else"),
        _ => println!("Not sure what the weather is.")
    }
    
    
    let children = 5;
    let married = true;
    
    match (children, married) {
        //(children, married) 
        //(c, m ) if married == false => println!("Not married with {} children ",children),
        (children, married) if married == false => println!("Not married with {} children", children),
        (c, m) if c == 0 && married == true  => println!("Married but with no children"),
        (c, m) if c == 5 && married == true  => println!("Married Test"),
        _ => println!("Some other type of marriage and children combination")
    }
    
}