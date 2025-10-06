fn sum(numbers: &[i32]) -> i32  {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn panicahhh(numbers: Vec<i32>) {
    for num in numbers {
        if num < 0 {
            panic!("Negative number found");
        }
        println!("Number: {}", num)
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let result = sum(&numbers);
    println!("the sum is {}", result);
    panicahhh(vec![1, 2, 3, 4, -5]);
}