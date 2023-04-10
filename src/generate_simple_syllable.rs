use rand::seq::SliceRandom;

pub fn generate_simple_syllable(
    consonant_inventory: Vec<&str>,
    vowel_inventory: Vec<&str>,
) -> String {
    let mut rng = rand::thread_rng();

    let mut generated_syllable = String::from("");

    let random_consonant: String = consonant_inventory.choose(&mut rng).unwrap().to_string();
    generated_syllable.push_str(&random_consonant);

    let random_vowel: String = vowel_inventory.choose(&mut rng).unwrap().to_string();
    generated_syllable.push_str(&random_vowel);

    generated_syllable.to_string()
}
