// tuples
// Vec<String>


fn main()  {
    
    // let my_tuple = (8, "Dave MacLeod", vec![8,9,10]);    
    // println!("{:?}",my_tuple);
    
    //my_tuple.asdfasdf();
    
    //let x = ();
    //tuple은 디버그 프린트 필요함
    //Vec은 똑같은 타입만 있어야함 
    //tuple은 타입 여러개 가능 
    
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8,9,10], 7.7);
    println!(
        "Inside the tuple is : First item: {:?}
        Second item: {}
        Third item: {:?}
        Fourth item: {:?}
        Fifth item: {:?}
        Sixth item: {}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );
    
    //Vec<&str, i32>
    //Destructuring
    //Structure 
    
    // let my_vec = vec![("Hey",9), ("Hello there", 877123)];
    // println!("{:?}",my_vec);
    
    let str_tuple = ("one","two","three");
    println!("{:?}",str_tuple);
    
    let (a, b, c) = str_tuple; 
    println!("{}, {}, {}", a, b, c);
    
    let (d, _, _) = str_tuple;
    println!("{}", d);
    
    let str_array = ["one", "two", "three"];
    let [e, _, _] = str_array;
    println!("Item e is {} ",e);
}
