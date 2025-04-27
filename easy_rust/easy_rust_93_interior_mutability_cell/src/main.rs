// interior mutability
// changing on the inside
// & immutable reference / shared reference 
// &mut mutable reference / unique reference 

// Cell- small room 
// RefCell
// Mutex
// RwLock 

// let mut x = 9;
// thread 1 { x+=1; }
// thread 2 { x+=1; }

use std::cell::Cell; 

#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

fn main() {
    
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true) 
    };

    println!("{super_phone_3000:?}");

    super_phone_3000.on_sale.set(false);

    println!("{super_phone_3000:?}");

}
