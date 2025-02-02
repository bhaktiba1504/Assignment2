fn main() {
    let numbers = vec![1, 2, 3, 4, 5]; // Heap-allocated vector (Rust automatically frees it)
    println!("Numbers: {:?}", numbers);

    let sum = sum_numbers(&numbers); // Borrowing (no ownership transfer)
    println!("Sum: {}", sum);
} 

fn sum_numbers(arr: &Vec<i32>) -> i32 {
    arr.iter().sum()
} // arr is borrowed, not moved, so it is not freed here
