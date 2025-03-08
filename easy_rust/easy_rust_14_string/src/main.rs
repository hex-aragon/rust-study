fn main() {
    //String = Sized type >> heap 
    //&str = dynamic type 레퍼런스 없이 하면 다이나믹? >> stack

    let my_name = "David".to_string();
    let other_name = String::from("David2");
    //growable + shrinkable 
    let mut my_other_name = "David3".to_string();
    my_other_name.push('!');
    println!("{}", my_other_name);
}
