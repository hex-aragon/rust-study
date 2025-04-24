// find - Option<Self::Item> "I'll try to get it for you"
// position - Option<usize> "I'll try to find the position for you"
// cycle 끝나지 않는 iterator ? 
//[1,2,3].iter().cycle()

fn main() {
    let even_odd = vec!["even", "odd"].into_iter().cycle();
 
    let even_odd_vec = (0..6) // Range are iterators 
        .zip(even_odd)
        .collect::<Vec<(i32, &str)>>();
 
    println!("{even_odd_vec:?}");




    let even_odd2 = vec!["even", "odd"];
 
    let even_odd_vec2 = (0..6) // Range are iterators 
        .zip(even_odd2.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();
 
    println!("{even_odd_vec2:?}");

    let even_odd3 = vec!["even", "odd"];
    let six_items = even_odd3.into_iter().cycle().take(6).collect::<Vec<_>>();
     println!("{six_items:?}");


}
