// destructuring
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

#[derive(Debug)]
struct Person2 {
    name: String,
    height: u8
}

impl Person2 {
    fn from_person(i: Person) -> Self {
        let Person { name, height, .. } = i;
        
        Self {
            name,
            height
        }
    }
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    
    //rust fmt 
    let Person {
        name: a,
        real_name: b,
        height: c,
        happiness: d,
    } = papa_doc;

    //위와   같이 구조분해를 하면 하단과 같이 할 수 있다.
    // println!("The call him {} but his real name is {}. He is {} cm tall and is he happy ? {}",
    // papa_doc.name, papa_doc.real_name, papa_doc.height, papa_doc.happiness);

    println!(
        "The call him {} but his real name is {}. He is {} cm tall and is he happy ? {}",
        a, b, c, d
    );
    
    let papa_doc2 = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    
    let person2 = Person2::from_person(papa_doc2);
    println!("Person2 type is : {:?}", person2);
    
}
