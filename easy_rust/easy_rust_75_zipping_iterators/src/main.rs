//map
//for_each 차이 체크 
//.zip

use std::collections::HashMap;

fn main() {
    let num_vec = vec![2,4,6];

    let new_thing = num_vec
        .iter() //2, 4, 6
        .enumerate() // (0, 2), (1, 4), (2, 6)
        .map(|(index, number)|{
            println!("The number at index {index} is {number}");
            0
        })
        .collect::<Vec<_>>();
    
    println!("{new_thing:?}");


    let num_vec2 = vec![1,3,5];
    num_vec2
        .iter() 
        .enumerate()
        .for_each(|tuple| {
            println!("The number at index {} is {} ", tuple.0, tuple.1);
        });

    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"]; //Vec<&str>

    let some_numbers2 = vec![0, 1, 2, 3, 4, 5];
    let some_words2 = vec!["zero", "one", "two", "three", "four", "five"]; //Vec<&str>

    let number_word_hashmap : HashMap<i32, &str> = some_numbers 
        .into_iter()
        .zip(some_words.into_iter())
        .collect();

    let result_str =  number_word_hashmap.get(&10).unwrap_or_else(|| {
       // println!("Uh oh, didn't work");
       println!("Help");
        &"no number"        
    });

    println!("{result_str}");


    let number_word_hashmap2 : HashMap<i32, &str> = some_numbers2 
        .into_iter()
        .zip(some_words2.into_iter())
        .collect();
    
    number_word_hashmap2.iter().for_each(|stuff|{
        println!("{stuff:?}");
    });
}
