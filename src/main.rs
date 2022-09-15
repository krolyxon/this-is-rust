fn main() {
    println!("This is Rust.");
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    match input.trim() {
        "yes" => {
            println!("yes, you are right. this is rust.");
        }
        _ => {
            println!("No, you wrong.");
        }
    }
}
