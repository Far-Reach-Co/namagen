use crate::gen_simple_syllable::gen_simple_syllable;
use rand::{random, seq::SliceRandom};

pub fn gen_saurian_name() -> String {
    let consonant_inventory = vec![
        "'", "b", "j", "d", "h", "w", "z", "ħ", "ṭ", "y", "k", "l", "m", "n", "s", "ƹ", "f", "ṣ",
        "q", "r", "š", "t", "θ", "x", "ð", "ḍ", "ẓ", "ɣ",
    ];
    let vowel_inventory = vec!["a", "i", "u", "r̩", "l̩"];
    let syllables_per_name = 3;

    let mut rng = rand::thread_rng();

    let mut generated_name = String::from("");

    let mut most_recent_grapheme = "";
    let mut two_vowels_or_consonants_in_a_row = false;
    let mut generated_syllables = 0;

    let random_consonant: String = consonant_inventory
        .choose(&mut rng)
        .unwrap()
        .to_string()
        .to_uppercase();
    generated_name.push_str(&random_consonant);
    most_recent_grapheme = "consonant";

    if generated_name == "'" {
        let random_vowel: String = vowel_inventory
            .choose(&mut rng)
            .unwrap()
            .to_string()
            .to_uppercase();
        generated_name.push_str(&random_vowel);
        most_recent_grapheme = "vowel";
    };

    if random() {
        let random_vowel: String = vowel_inventory.choose(&mut rng).unwrap().to_string();
        generated_name.push_str(&random_vowel);
        if most_recent_grapheme == "vowel" {
            two_vowels_or_consonants_in_a_row = true
        } else {
            most_recent_grapheme = "vowel";
        }
    } else {
        let random_consonant: String = consonant_inventory
            .choose(&mut rng)
            .unwrap()
            .to_string()
            .to_uppercase();
        generated_name.push_str(&random_consonant);
        if most_recent_grapheme == "consonant" {
            two_vowels_or_consonants_in_a_row = true
        } else {
            most_recent_grapheme = "consonant";
        }
    }

    generated_name.to_string()
}
