fn main() {
    println!("is this rust? ");
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    match input.trim() { // trimming to remove newline "\n"
        "yes" => {
            println!("yes, you are right. this is rust.");
        }
        _ => {
            println!("No, you wrong.");
        }
    }
}
