fn main() {
    // for i in 1..=10 {
    //     println!("i = {}", i);
    // }
    
    // for i in (1..=5).rev() {
    //     println!("i = {}", i);
    // };

    // let numbers = vec![1, 2, 3, 4, 5];
    // for n in numbers {
    //     println!("{}", n);
    // }

    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i = {}", i);
        if i == 7 {
            break;
        }
    }
}
