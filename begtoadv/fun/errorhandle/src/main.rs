fn main() {
    let file = File::open("non_existent_file.txt");
    let file = match filr {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::notFound => {
                    panic!("file not found {}", error)
                }
                _ => {
                    panic!("Error opening file {}", error)
                }
            }
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
    println!("Hello, world!");
}
