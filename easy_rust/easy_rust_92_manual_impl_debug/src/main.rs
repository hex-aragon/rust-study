//Debug again 

//External code 
mod client {
    pub struct InternetClient {
        pub client_id : u32  //Other stuff 
    }
}

use client::InternetClient;


struct Customer<'a> {
    money: u32, 
    name: &'a str,
    client: &'a InternetClient
}

use std::fmt;

impl fmt::Debug for Customer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Customer")
        .field("money", &self.money)
        .field("name", &self.name)
        .field("client", &"Client")
        .finish()
    }
}


fn main(){
    let client = client::InternetClient {
        client_id:0
    };

    let customer1 = Customer{
        money: 6875,
        name: "Billy",
        client: &client 
    };

    println!("{customer1:?}");
}