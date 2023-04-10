mod gen_mamobibu_name;
pub mod gen_syllable_based_name;
use crate::gen_syllable_based_name::gen_syllable_based_name;
use gen_mamobibu_name::gen_mamobibu_name;
use wasm_bindgen::prelude::*;
pub mod gen_simple_syllable;

// Just a reminder to myself of how to do this
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> String {
    "Choose a language to generate a name in! Please enter the name of a language or enter 'List' for a list of available languages".to_string()
}

#[wasm_bindgen]
pub fn namagen(language_input: &str) -> String {
    let language_list = "Mamobibu, Saurian, Wyr";

    let binding = language_input.to_lowercase();
    let language_input = binding.trim();

    match language_input {
        "list" => language_list.to_string(),
        "mamobibu" => {
            "Generating a name in the Mamobibu language: ".to_owned()
                + &crate::gen_mamobibu_name::gen_mamobibu_name()
        }
        "saurian" => "Generating a name in the Saurian language... NOT YET IMPLEMENTED".to_string(),

        "wyr" => "Generating a name in the Wyr language... NOT YET IMPLEMENTED".to_string(),
        _ => "Language not found! Enter 'List' for a list of available langauges".to_string(),
    }
}

#[wasm_bindgen]
pub fn gen_custom_simple_name(
    consonant_inventory: String,
    vowel_inventory: String,
    syllables_per_name: i32,
) -> String {
    let consonant_inventory: Vec<&str> = consonant_inventory.split(',').collect();
    let vowel_inventory: Vec<&str> = vowel_inventory.split(',').collect();
    gen_syllable_based_name(
        consonant_inventory.to_owned(),
        vowel_inventory.to_owned(),
        syllables_per_name,
    )
}

#[wasm_bindgen]
pub fn mamobibu() -> String {
    gen_mamobibu_name()
}
