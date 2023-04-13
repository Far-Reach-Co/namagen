mod gen_mamobibu_name;
mod gen_saurian_name;
mod gen_simple_name;
mod gen_simple_syllable;
mod gen_syllabic_name;
mod get_saurian_alphabet;
mod trans_to_basic_latin;
use crate::gen_syllabic_name::gen_syllabic_name;
use gen_mamobibu_name::gen_mamobibu_name;
use gen_saurian_name::gen_saurian_name;
use gen_simple_name::gen_simple_name;
use get_saurian_alphabet::get_saurian_alphabet;
use trans_to_basic_latin::trans_to_basic_latin;
use wasm_bindgen::prelude::*;

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
pub fn custom_simple_name(
    consonant_inventory: String,
    vowel_inventory: String,
    syllables_per_name: i32,
) -> String {
    let consonant_inventory: Vec<&str> = consonant_inventory.split(',').collect();
    let vowel_inventory: Vec<&str> = vowel_inventory.split(',').collect();
    let custom_simple_name = gen_simple_name(
        consonant_inventory.to_owned(),
        vowel_inventory.to_owned(),
        syllables_per_name,
    );
    "custom_simple_name:".to_string() + &custom_simple_name
}

#[wasm_bindgen]
pub fn custom_syllabic_name(
    consonant_inventory: String,
    vowel_inventory: String,
    syllables_per_name: i32,
) -> String {
    let consonant_inventory: Vec<&str> = consonant_inventory.split(',').collect();
    let vowel_inventory: Vec<&str> = vowel_inventory.split(',').collect();
    let custom_syllabic_name = gen_syllabic_name(
        consonant_inventory.to_owned(),
        vowel_inventory.to_owned(),
        syllables_per_name,
    );
    "custom_syllabic_name:".to_string() + &custom_syllabic_name
}

#[wasm_bindgen]
pub fn mamobibu() -> String {
    let mamobibu_name = gen_mamobibu_name();
    "mamobibu_name:".to_string() + &mamobibu_name
}

#[wasm_bindgen]
pub fn saurian() -> String {
    let saurian_name = gen_saurian_name();
    let saurian_name_basic_latin =
        trans_to_basic_latin(&saurian_name, "Saurian", get_saurian_alphabet());
    "saurian_name:".to_string()
        + &saurian_name
        + ",saurianNameBasicLatin:"
        + &saurian_name_basic_latin
}
