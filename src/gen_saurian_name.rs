use crate::gen_syllable_based_name::gen_syllable_based_name;

pub fn gen_saurian_name() -> String {
    let consonant_inventory = vec!["'", "b", "j", "d", "h", "w", "z", "ħ", "ṭ" "y", "k", "l", "m", "n", "s", "ƹ", "f", "ṣ", "q", "r", "š", "t", "θ", "x", "ð", "ḍ", "ẓ", "ɣ"];
    let vowel_inventory = vec!["a", "i", "u"];
    let syllables_per_name = 3;

    gen_syllable_based_name(consonant_inventory, vowel_inventory, syllables_per_name)
}
