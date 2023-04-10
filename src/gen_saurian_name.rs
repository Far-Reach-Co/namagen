use crate::gen_simple_syllable::gen_simple_syllable;
use rand::seq::SliceRandom;

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

    generated_name.to_string()
}
