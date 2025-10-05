
fn own_vec(mut vector: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    new_vec.push(10);
    new_vec
    // vector.push(10);
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: &String) {
    println!("{}", s)
} 


fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello Rust!")
    println!("Hello, world!");


own_integer(my_int);
println!("{}", my_int);

own_string(&my_string);
println!("{}", my_string);

let new_vec = own_vec(&my_vec);
println!("{:?}", new_vec);
}