use std::fmt::Debug;
use std::ops::Add;

//traits
//interface

//다른 언어의 interface 느낌이 rust의 traits다. 

//struct User //thing
//enum Months //choices
//trait // verbs /adjectives >> 동사 느낌 

//#[derive(Add)]
#[derive(Clone, Copy, Debug)]
struct ThingsToAdd {
    first_thing: u32,
    second_thing: f32
}

//Debug 같은 게 trait
#[derive(Debug)]
struct MyStruct {
    number : usize
}

fn print_as_debug<T: Debug>(input: T) {
    println!("{input:?}");
}

//Implement the Add trait for ThingsToAdd 
impl Add for ThingsToAdd {
    type Output = ThingsToAdd; //The return type of the addition

    fn add(self, other: Self) -> Self::Output {
        ThingsToAdd {
            first_thing: self.first_thing + other.first_thing,
            second_thing: self.second_thing + other.second_thing
        }
    }
}

fn main() {

    let my_thing = ThingsToAdd {
        first_thing: 32,
        second_thing: 8.8
    };

    let second_thing = ThingsToAdd {
        first_thing: 32,
        second_thing: 8.8
    };


    let sum = my_thing + second_thing;
    println!("{:?}",sum);
}