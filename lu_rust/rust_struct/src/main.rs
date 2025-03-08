#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

struct RectangleBuilder {
    width: Option<u32>,
    height: Option<u32>,
}

impl RectangleBuilder {
    fn new() -> Self {
        RectangleBuilder {
            width: None,
            height: None,
        }
    }

    fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    fn build(self) -> Result<Rectangle, &'static str> {
        let width = self.width.ok_or("Width is required")?;
        let height = self.height.ok_or("Height is required")?;
        Ok(Rectangle { width, height })
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    let square = Rectangle::square(20);

    println!("rect1 is {:?}", rect1);
    println!("The area of rect1 is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square: {:?}", square);

    let rect4 = RectangleBuilder::new()
        .width(35)
        .height(55)
        .build();

    match rect4 {
        Ok(r) => println!("Built rectangle: {:?}", r),
        Err(e) => println!("Error building rectangle: {}", e),
    }

    let rect5 = RectangleBuilder::new()
        .width(40)
        .build();

    match rect5 {
        Ok(r) => println!("Built rectangle: {:?}", r),
        Err(e) => println!("Error building rectangle: {}", e),
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller(){
            let larger  = Rectangle::new(8,7);
            let smaller = Rectangle::new(5,7);
            assert!(larger.can_hold(&smaller));
        }

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle::new(8,7);
            let smaller = Rectangle::new(5,1);
            assert!(!smaller.can_hold(&larger));
        }

        #[test]
        fn area_calculation(){
            let rect = Rectangle::new(5,5);
            assert_eq!(rect.area(),25);
        }

        #[test]
        fn square_creation() {
            let square = Rectangle::square(5);
            assert_eq!(square.width, 5);
            assert_eq!(square.height, 5);
        }

        #[test]
        fn builder_success() {
            let rect = RectangleBuilder::new()
                .width(10)
                .height(20)
                .build();
            assert!(rect.is_ok());
            let rect = rect.unwrap();
            assert_eq!(rect.width,10);
            assert_eq!(rect.height,20);
        }

        #[test]
        fn builder_failure() {
            let rect = RectangleBuilder::new()
                .width(10)
                .build();
            assert!(rect.is_err());
        }
    }
}
