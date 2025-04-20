//Iterator = a collection of things that you can call .next() on 
//for loop 쓸 때 위의 함수를 많이 씀 

fn main() {
    // 3개 함수를 많이 사용한다. 
    // .iter() - iterator of references &T
    // .iter_mut() - iterator of mutable reference &mut T
    // .into_iter() - consuming iterator >> 레퍼런스를 안봐도 된다 ?

    let vector1 = vec![1,2,3];
    let vector1_a = vector1.iter().map( |x| x + 1).collect::<Vec<i32>>(); //vector1 값이 아직 있고 새로운 밸류를 만든다. 
    let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>(); //vector1이 소멸된다. 
    
    //이 방식도 가능
    //let vector1_b : Vec<i32> = vector1.into_iter().map(|x| x * 10).collect();

    let mut vector2 = vec![10,20,30];
    //for_each = for loop랑 같다. 레퍼런스 
    vector2.iter_mut().for_each(|num| *num += 100);
    

    println!("{vector1_a:?}");
    println!("{vector1_b:?}");
    println!("{vector2:?}");
     //println!("{vector1:?}"); into_iter()에서 소멸됨 
}
