fn sum(numbers: &[i32]) -> i32  {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let result = sum(&numbers);
    println!("the sum is {}", result);
}