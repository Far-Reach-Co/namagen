use crate::generate_syllable_based_name::generate_syllable_based_name;

pub fn mamobibu() {
    let consonant_inventory = "mb";
    let vowel_inventory = "aeiou";
    let syllables_per_name = 4;
    println!("Generating a name in the Mamobibu language...");

    generate_syllable_based_name(
        consonant_inventory.to_owned(),
        vowel_inventory.to_owned(),
        syllables_per_name,
    )
}
