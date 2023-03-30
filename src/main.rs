use std::io;

fn main() {
    println!("Hello, world!");
    println!("Choose a language to generate a name in!");
    println!("Please input the name of a language");
    println!("Input 'List' for a list of available langauges");

    let mut current_language = String::new();

    io::stdin()
        .read_line(&mut current_language)
        .expect("Failed to read line");

    println!("Generating a name in the {current_language} language...")
}
