use rand::seq::IteratorRandom;

pub fn generate_syllable_based_name(
    consonant_inventory: String,
    vowel_inventory: String,
    syllables_per_name: i32,
) -> String {
    let mut rng = rand::thread_rng();

    let mut generated_name = String::from("");

    let random_consonant: String = consonant_inventory
        .chars()
        .choose(&mut rng)
        .unwrap()
        .to_string()
        .to_uppercase();
    generated_name.push_str(&random_consonant);

    let mut generated_syllables = 0;
    while generated_syllables < syllables_per_name {
        let random_vowel: String = vowel_inventory
            .chars()
            .choose(&mut rng)
            .unwrap()
            .to_string();
        generated_name.push_str(&random_vowel);

        let random_consonant: String = consonant_inventory
            .chars()
            .choose(&mut rng)
            .unwrap()
            .to_string();
        generated_name.push_str(&random_consonant);

        generated_syllables += 1
    }

    generated_name.to_string()
}
