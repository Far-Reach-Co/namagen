use crate::generate_syllable_based_name::generate_syllable_based_name;

pub fn gen_saurian_name() -> String {
    let consonant_inventory = vec!["'", "b", "j", "d", "h", "w", "z", "ħ", "ṭ" "y", "k", "l", "m", "n", "s", "3", "f", "ṣ", "q", "r", "š", "t", "θ", "x", "ð", "ḍ", "ẓ", "ɣ"];
    let vowel_inventory = vec!["a", "i", "u"];
    let syllables_per_name = 3;

    "Generating a name in the Mamobibu language: ".to_owned()
        + &generate_syllable_based_name(consonant_inventory, vowel_inventory, syllables_per_name)
}
