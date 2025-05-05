//Any trait


//downcast_ref::<Book>
//downcast_ref::<Magazine>
//trait PublishingMaterial {}
//Box dyn PublishingMaterial


use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

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

fn book() -> Book {
    Book {
        name: "My Book".to_string(),
    }
}

fn magazine() -> Magazine {
    Magazine {
        name: "My Magazine".to_string(),
    }
}

fn main() {
    let (sender, receiver) = channel();

    let s1 = sender.clone();
    let s2 = sender.clone();

    thread::spawn(move || {
        //take by value
        //sleepy(10000);
        for _ in 0..5 {
            sleepy(100);
            s1.send(book()).unwrap();
        }
        // s1.send(8).unwrap();
    });

    thread::spawn(move || {
        //sleepy(10000);
        //s2.send(8).unwrap();
        for _ in 0..5 {
            sleepy(50);
            s2.send(magazine()).unwrap();
        }
    });

    // println!("{}", receiver.recv().unwrap()); //blocking
    // println!("{}", receiver.recv().unwrap());
    //println!("{:?}", receiver.try_recv()); //blocking
    //println!("{:?}", receiver.try_recv());
    println!("All done!");

    println!("{:?}", receiver.recv_timeout(Duration::from_millis(500))); //blocking
    println!("{:?}", receiver.recv_timeout(Duration::from_millis(500))); //blocking
}
