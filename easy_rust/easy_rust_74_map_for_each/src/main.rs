// map 
// for_each

fn main() {

    let num_vec  = vec![2, 4, 6];
    let double_vec = num_vec
        .iter()
        .map(|number| number * 2 )
        .map(|number| number * 2 )
        .map(|number| number * 2 );
        //.collect::<Vec<i32>>();
        //.collect::<Vec<_>>(); 이렇게도 가능 
    
    num_vec 
        .iter() //2,4,6
        .enumerate() //(0,2), (1,4), (2,6)
        .for_each(|(index, number)| { // == for loop 
            println!("The number at index {index} is {number}");
        });
    
    println!("{double_vec:?}");

    let num_vec2 = vec![1,3,5];
    num_vec2
        .iter() 
        .enumerate()
        .for_each(|tuple| {
            //println!("The tuple is : {tuple:?}");
            println!("The number at index {} is {} ", tuple.0, tuple.1);
        });


 let num_vec3 = vec![1,3,5];
    num_vec3
        .iter() 
        .enumerate()
        .for_each(|(index, number)| {
            println!("The number at index {index} is {number} ");
        });

}
