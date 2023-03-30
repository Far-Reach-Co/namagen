use std::io;

fn main() {
    println!("Choose a language to generate a name in!");
    println!("Please enter the name of a language");
    println!("Enter 'List' for a list of available languages");

    let mut current_language = String::new();

    let language_list = ["Saurian", "Wyr"];

    io::stdin()
        .read_line(&mut current_language)
        .expect("Failed to read line");

    let current_language: &str = &current_language.trim();

    match current_language {
        "List" => println!("{:?}", language_list),
        "Saurian" => println!("Generating a name in the {current_language} language..."),
        "Wyr" => println!("Generating a name in the {current_language} language..."),
        _ => println!("Language '{current_language}' not found! Enter 'List' for a list of available langauges"),
    }
}
