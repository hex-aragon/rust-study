//Chaining methods and functional style
// .next()
//.next() Some(T) None
fn main() {
    // let my_variable = SomeType {};
    //순서대로 함수 실행시 깔끔한 코드가 나옴
    //my_variable.iter().take(8).collect;

    //일반적인 프로그래밍
    let mut new_vec = Vec::new();
    let mut counter = 1;

    while counter < 11 {
        new_vec.push(counter);
        counter += 1;
    }

    println!("{new_vec:?}");
    //[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    //러스트스러운 방식
    //let new_vec2 = (1..=10).collect::<Vec<i32>>(); //방법 1
    let new_vec2 = (1..=10).collect::<Vec<_>>(); //방법 2
    println!("{:?}", new_vec2);
    //[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //into_iter()는 콜렉션에서 1개씩 꺼내는 함수
    //skip 은 무시하고 진행
    //take 4개를 반환
    //let new_vec3 = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    let new_vec3 = my_vec
        .into_iter()
        .skip(1)
        .skip(1)
        .skip(1)
        .take(4)
        .collect::<Vec<i32>>();
    println!("{new_vec3:?}",)
    //[4, 5, 6, 7]
}
