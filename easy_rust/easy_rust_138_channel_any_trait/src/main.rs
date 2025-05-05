//Any trait


//downcast_ref::<Book>
//downcast_ref::<Magazine>
//trait PublishingMaterial {}
//Box dyn PublishingMaterial


use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::any::Any;

fn sleepy(time: u64) {
    sleep(Duration::from_millis(time));
}

#[derive(Debug)]
struct Book {
    name: String,
}

#[derive(Debug)]
struct Magazine {
    name: String,
}

fn book() -> Box<dyn Any + Send > { //turn to trait object
    let book =  Book {
        name: "My Book".to_string(),
    };
    Box::new(book)
}

fn magazine() -> Box<dyn Any + Send> {
    let magazine = Magazine {
        name: "My Magazine".to_string(),
    };
    Box::new(magazine)
}

fn main() {
    let (sender, receiver) = channel();

    let s1 = sender.clone();
    let s2 = sender.clone();

    thread::spawn(move || {
        //take by value
        //sleepy(10000);
        for _ in 0..5 {
            sleepy(1);
            s1.send(book()).unwrap();
        }
        // s1.send(8).unwrap();
    });

    thread::spawn(move || {
        //sleepy(10000);
        //s2.send(8).unwrap();
        for _ in 0..5 {
            sleepy(1);
            s2.send(magazine()).unwrap();
        }
    });

    // println!("{}", receiver.recv().unwrap()); //blocking
    // println!("{}", receiver.recv().unwrap());
    //println!("{:?}", receiver.try_recv()); //blocking
    //println!("{:?}", receiver.try_recv());
    

    // println!("{:?}", receiver.recv_timeout(Duration::from_millis(500))); //blocking
    // println!("{:?}", receiver.recv_timeout(Duration::from_millis(500))); //blocking
    while let Ok(any_type) = receiver.recv() {
        println!("{:?}", any_type);
        if let Some(book) = any_type.downcast_ref::<Book>(){
            println!("Got a book: {book:?}");
        } else if let Some(magazine) = any_type.downcast_ref::<Magazine>(){
            println!("Got a magazine: {magazine:?}");
        } else {
            println!("Expected a magazine or a book, what's going on");
        }
    }
    println!("All done!");

}
