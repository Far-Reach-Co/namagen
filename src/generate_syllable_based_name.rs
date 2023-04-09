use crate::generate_simple_syllable::generate_simple_syllable;
use rand::seq::SliceRandom;

pub fn generate_syllable_based_name(
    consonant_inventory: Vec<&str>,
    vowel_inventory: Vec<&str>,
    syllables_per_name: i32,
) -> String {
    let mut rng = rand::thread_rng();

    let mut generated_name = String::from("");

    let random_consonant: String = consonant_inventory
        .choose(&mut rng)
        .unwrap()
        .to_string()
        .to_uppercase();
    generated_name.push_str(&random_consonant);

    let mut generated_syllables = 0;
    while generated_syllables < syllables_per_name {
        let generated_syllable: String =
            generate_simple_syllable(consonant_inventory.clone(), vowel_inventory.clone());
        generated_name.push_str(&generated_syllable);

        generated_syllables += 1
    }

    generated_name.to_string()
}
