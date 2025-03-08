trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog{
    fn make_sound(&self) -> String{
        String::from("Woof!")
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String{
        String::from("Meow!")
    }
}

fn animal_sounds(animals: Vec<Box<dyn Animal>>) {
    for animal in animals{
        println!("The animal says: {}", animal.make_sound());
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    animal_sounds(animals);
    

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_dog_sound(){
            let dog = Dog;
            assert_eq!(dog.make_sound(), "Woof!");
        }

        #[test]
        fn test_cat_sound(){
            let cat = Cat;
            assert_eq!(cat.make_sound(), "Meow!");
        }

        #[test]
        fn test_animal_sounds(){
            let animals: Vec<Box<dyn Animal>> = vec![
                Box::new(Dog),
                Box::new(Cat),
            ];
            animal_sounds(animals);
        }
    }
}
