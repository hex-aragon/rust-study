#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

fn main() {

    let p1 = Point {x: 1, y: 2};
    let p2 = p1;
    println!("p1: {:?}, p2:{:?}", p1, p2);

    let l1 = Line{start: Point {x: 0, y: 0}, end: Point{x:5, y: 5}};
    let l2 = l1.clone();
    println!("l1: {:?}", l1);
    println!("l2: {:?}",l2);

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_point_copy(){
            let p1 = Point {x: 1, y: 2};
            let p2 = p1;
            assert_eq!(p1.x, p2.x);
            assert_eq!(p1.y, p2.y);
        }

        #[test]
        fn test_line_clone(){
            let l1 = Line {
                start: Point {x: 0, y:0},
                end : Point {x: 5, y:5},
            };
            let l2 = l1.clone();
            assert_eq!(l1.start.x, l2.start.x);
            assert_eq!(l1.start.y, l2.start.y);
            assert_eq!(l1.end.x, l2.end.x);
            assert_eq!(l1.end.y, l2.end.y);
        }
    }

}
