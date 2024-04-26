use std::vec;

fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 3, 4, 5];
    let sum: i32 = nums.iter().sum();
    print!("Sum: {}", sum)
}
