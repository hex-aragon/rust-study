fn main() {
    println!("Hello, world!");

    //정적 디스패치 예제
    println!("Static dispatch:");
    static_dispatch("hello");
    static_dispatch(5);

    //동적 디스패치 예제
    println!("\nDynamic dispatch:");
    let dog = Dog; 
    let cat = Cat; 
    animal_sound(&dog);
    animal_sound(&cat);

    //함수 포인터 예제
    println!("\nFunction pointer:");
    let result = do_twice(add_one, 5);
    println!("Result : {}", result);

    //클로저 예제
    println!("\nClosure:");
    let x = 4; 
    let equal_to_x = |z| z == x;
    let y = 4; 
    println!("Is y equal to x? {}", equal_to_x(y));


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_static_dispatch(){
            static_dispatch("test");
            static_dispatch(10);
            println!("static_dispatch test ok")
            //컴파일 되면 성공 
        }

        #[test]
        fn test_dynamic_dispatch(){
            let dog = Dog; 
            let cat = Cat; 
            animal_sound(&dog);
            animal_sound(&cat);
            //컴파일 되면 성공 
            println!("dynamic dispatch test ok");
        }

        #[test]
        fn test_function_pointer(){
            assert_eq!(do_twice(add_one,5),7);
        }

        #[test]
        fn test_closure(){
            let x = 4; 
            let equal_to_x = |z| z == x;
            assert!(equal_to_x(4));
            assert!(!equal_to_x(5));
        }
    }
}

/** 정적 디스패치 
 * 구조체나 열거형 등에 대한
 * 자세한 설명을 쓸 때 사용한다
 *
 * # 필드
 *
 * * `x` - x 좌표
 * * `y` - y 좌표
 */
fn static_dispatch<T: std::fmt::Display>(t: T){
    println!("{}",t);
}


trait Animal {
    fn make_sound(&self);
}

struct Dog; 
impl Animal for Dog {
    fn make_sound(&self){
        println!("Woof!");
    }
}


struct Cat; 
impl Animal for Cat {
    fn make_sound(&self){
        println!("Meow!");
    }
}

fn animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}

fn add_one(x : i32) -> i32 {
    x + 1
}

fn do_twice(f : fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}