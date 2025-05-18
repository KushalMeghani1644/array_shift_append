use std::io;
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    println!("Initial array: {:?}", arr);
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid number");
    let new_num: i32 = input.trim().parse().expect("Please enter a valid number");
    for i in 0..arr.len() - 1 {
        arr[i] = arr[i + 1];
    }
    arr[arr.len() - 1] = new_num;
    println!("Updated array: {:?}", arr);
}
