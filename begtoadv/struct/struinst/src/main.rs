struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> User {
        User {
            username,
            email,
            uri,
            active: true,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
