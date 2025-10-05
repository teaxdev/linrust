fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }

    println!("Sum of numbers is: {} ", sum);

    if sum % 2 ==0 {
        println!("even");
    } else {
        println!("odd");
    }
}

fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    // result.to_string()
    result.expect("oops").to_string()
}


fn main() {
    process_numbers(&[1, 2, 3]);
    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}", chunk)
}
