//map
//and_then 
//.map(|some_thing| some_thing + 1)


fn main() {
    
    // let some_output : Option<Vec<i32>> = None; // = Some(vec![8,9,10]);
     let some_output = Some(vec![8,9,10]);
     
     let first = 
         some_output
         .clone()
         .map(|some_vec| some_vec.iter().map(|num| num + 1).collect::<Vec<_>>());
 
     //let second = some_output.map(|some_vec| match some_vec.len(){ // Some(Some([8,9,10]))
     let second = some_output.and_then(|some_vec| match some_vec.len(){ // [8,9,10]
         0 => None,
         1 => Some(vec![some_vec[0]]),
         _ => Some(some_vec)
     }); // flatten
 
     let some_output2 = Some(vec![8,9,10]);
     
     let third = some_output2.map(|some_vec| match some_vec.len(){ //Some(Some([8, 9, 10]))
         0 => None,
         1 => Some(vec![some_vec[0]]),
         _ => Some(some_vec)
     }).flatten();
     //.flatten에 따라 달라짐, 
     
     println!("{first:?}");
 
     println!("{second:?}");
 
 
     println!("{third:?}");
 }
 