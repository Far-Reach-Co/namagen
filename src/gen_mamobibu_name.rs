use crate::generate_syllable_based_name::generate_syllable_based_name;

pub fn gen_mamobibu_name() -> String {
    let consonant_inventory = vec!["m", "b"];
    let vowel_inventory = vec!["a", "e", "i", "o", "u"];
    let syllables_per_name = 4;

    generate_syllable_based_name(consonant_inventory, vowel_inventory, syllables_per_name)
}
