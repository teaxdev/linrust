struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

struct Point(i32, i32, i32);
fn main() {
    let username = String::from("Alfred");
    let email = String::from("Alfred@gmail.com");
    let uri = String::from("Alfred.com");
    let active = true;

    let user = User {
        username,
        email,
        uri,
        active,
    };
    let my_point = Point(10, 20, 30);
    println!("Points: {}", my_point.0);
}
