pub mod generate_syllable_based_name;
mod mamobibu;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> String {
    "Choose a language to generate a name in! Please enter the name of a language or enter 'List' for a list of available languages".to_string()
}

#[wasm_bindgen]
pub fn main(language_input: &str) -> String {
    {
        alert(&format!("Hello, World!"))
    };

    let language_list = "Mamobibu, Saurian, Wyr";

    let language_input = language_input.trim();

    match language_input {
        "List" => language_list.to_string(),
        "Mamobibu" => crate::mamobibu::mamobibu(),
        "Saurian" => {
            "Generating a name in the {language_input} language... NOT YET IMPLEMENTED".to_string()
        }

        "Wyr" => {
            "Generating a name in the {language_input} language... NOT YET IMPLEMENTED".to_string()
        }
        _ => {
            "Language '{language_input}' not found! Enter 'List' for a list of available langauges"
                .to_string()
        }
    }
}
