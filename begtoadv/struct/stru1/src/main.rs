#[derive(Debug)]

struct Person {
    fname: String,
    lname: String,
    age: Option<u8>,
}

fn main() {
    let alfred = Person {
        fname: "Jimmy".to_string(),
        lname: "P".to_string(),
        age: None,
    };
    

    println!("your name: {}", alfred.fname);
    println!("your name: {:?}", alfred.age);
}
