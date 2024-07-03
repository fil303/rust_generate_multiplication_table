use std::io;
fn main() {
    println!("Enter a number: ");
    let input = io::stdin();
    let mut number = String::new();
    input.read_line(&mut number).unwrap();
    let number: Result<u32, std::num::ParseIntError> = number.trim().parse();
    let number: u32 = number.unwrap();

    generate_namta(number);
    // generate_namta(number.clone().unwrap());
    // match number {
    //     Ok(n) => generate_namta(n),
    //     Err(err) => println!("Error: Invalid input. Please enter a number. (Error: {})", err),
    // }

    println!("last line: input value {}", number);
}

fn generate_namta(number: u32)
{
    for i in 1..=10 {
        println!("{} X {} = {}", number, i, number * i);
    }
}
