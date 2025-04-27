//multiple threads!

use std::thread;

fn main() {

    let handle = thread::spawn(|| {
        println!("I am printing something");
    });

    for _ in 0..10 {
        let _x = 8;
        println!("{_x}");
    }

    handle.join().unwrap();


     for _ in 0..10 {
        std::thread::spawn(|| {
        println!("I am printing something");
    });
    }

    let join_handle = thread::spawn(|| {
        println!("I am printing join handle");
    });

    join_handle.join(); //wait 

    let mut join_vec = vec![];

    for _ in 0..10 {
        let handle2 = thread::spawn(||{ //stop
            println!("I am printing test")
        });
        join_vec.push(handle2);
    }

    //방법 1 > 러스트 많이 쓰면 아래와 같이 사용할 수 도 있음 
    //join_vec.into_iter().for_each(|handle2| handle2.join().unwrap());

    //방법 2 
    for handle2 in join_vec {
        handle2.join().unwrap();
    }


}
