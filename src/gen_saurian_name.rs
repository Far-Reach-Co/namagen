use rand::Rng;
use rand::{random, seq::SliceRandom, thread_rng};

pub fn gen_saurian_name() -> String {
    let consonant_inventory = vec![
        "'", "p", "j", "d", "h", "w", "z", "ħ", "ṭ", "y", "k", "l", "n", "s", "ƹ", "f", "ṣ", "q",
        "r", "š", "t", "θ", "x", "ð", "ḍ", "ẓ", "ɣ",
    ];
    let vowel_and_syllabic_inventory = vec!["a", "i", "u", "f̩", "l̩", "r̩", "s̩", "š̩"];
    let vowel_inventory = vec!["a", "i", "u"];
    let mut rng = thread_rng();
    let syllables_per_name = rng.gen_range(1..4);
    let mut most_recent_grapheme = "";
    let mut two_vowels_or_consonants_in_a_row = false;
    let mut generated_syllables = 0;
    let mut generated_name = String::from("");

    // Generate first letter
    if random() {
        let random_consonant: String = consonant_inventory
            .choose(&mut rng)
            .unwrap()
            .to_string()
            .to_uppercase();
        generated_name.push_str(&random_consonant);
        most_recent_grapheme = "consonant";

        if generated_name == "'" {
            generated_name = String::from("");
            let random_vowel: String = vowel_inventory
                .choose(&mut rng)
                .unwrap()
                .to_string()
                .to_uppercase();
            generated_name.push_str(&random_vowel);
            most_recent_grapheme = "vowel";
        };
    } else if random() && random() {
        let random_vowel: String = vowel_and_syllabic_inventory
            .choose(&mut rng)
            .unwrap()
            .to_string()
            .to_uppercase();
        generated_name.push_str(&random_vowel);
        most_recent_grapheme = "vowel";
    } else {
        let random_vowel: String = vowel_inventory
            .choose(&mut rng)
            .unwrap()
            .to_string()
            .to_uppercase();
        generated_name.push_str(&random_vowel);
        most_recent_grapheme = "vowel";
    }

    // If first letter is a consonant, add second letter as a vowel (so we don't have names that start with two consonants like "Tdeneb")
    if most_recent_grapheme == "consonant" {
        if random() && random() {
            let random_vowel: String = vowel_and_syllabic_inventory
                .choose(&mut rng)
                .unwrap()
                .to_string();
            generated_name.push_str(&random_vowel);
            generated_syllables += 1;
            most_recent_grapheme = "vowel";
        } else {
            let random_vowel: String = vowel_inventory.choose(&mut rng).unwrap().to_string();
            generated_name.push_str(&random_vowel);
            generated_syllables += 1;
            most_recent_grapheme = "vowel";
        }
    }

    // Add letters until syllable count is reached
    while generated_syllables < syllables_per_name && generated_name.chars().count() < 8 {
        if two_vowels_or_consonants_in_a_row == true {
            if most_recent_grapheme == "consonant" {
                if random() && random() {
                    let random_vowel: String = vowel_and_syllabic_inventory
                        .choose(&mut rng)
                        .unwrap()
                        .to_string();
                    generated_name.push_str(&random_vowel);
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                    two_vowels_or_consonants_in_a_row = false;
                } else {
                    let random_vowel: String =
                        vowel_inventory.choose(&mut rng).unwrap().to_string();
                    generated_name.push_str(&random_vowel);
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                    two_vowels_or_consonants_in_a_row = false;
                }
            } else {
                let random_consonant: String =
                    consonant_inventory.choose(&mut rng).unwrap().to_string();
                generated_name.push_str(&random_consonant);
                most_recent_grapheme = "consonant";
                two_vowels_or_consonants_in_a_row = false;
            }
        } else {
            if random() {
                let random_consonant: String =
                    consonant_inventory.choose(&mut rng).unwrap().to_string();
                generated_name.push_str(&random_consonant);
                if most_recent_grapheme == "consonant" {
                    two_vowels_or_consonants_in_a_row = true
                } else {
                    most_recent_grapheme = "consonant";
                }
            } else if random() && random() {
                let random_vowel: String = vowel_and_syllabic_inventory
                    .choose(&mut rng)
                    .unwrap()
                    .to_string();
                generated_name.push_str(&random_vowel);
                if most_recent_grapheme == "consonant" {
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                } else {
                    two_vowels_or_consonants_in_a_row = true
                }
            } else {
                let random_vowel: String = vowel_inventory.choose(&mut rng).unwrap().to_string();
                generated_name.push_str(&random_vowel);
                if most_recent_grapheme == "consonant" {
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                } else {
                    two_vowels_or_consonants_in_a_row = true
                }
            }
        }
    }

    // Potentially add a final grapheme or two
    if random() {
        let random_consonant: String = consonant_inventory.choose(&mut rng).unwrap().to_string();
        generated_name.push_str(&random_consonant);
        most_recent_grapheme = "consonant";
    } else if random() && random() {
        let random_vowel: String = vowel_and_syllabic_inventory
            .choose(&mut rng)
            .unwrap()
            .to_string();
        generated_name.push_str(&random_vowel);
    } else {
        let random_vowel: String = vowel_inventory.choose(&mut rng).unwrap().to_string();
        generated_name.push_str(&random_vowel);
    }

    if most_recent_grapheme == "vowel" && random() {
        let random_consonant: String = consonant_inventory.choose(&mut rng).unwrap().to_string();
        generated_name.push_str(&random_consonant);
    }

    // Return generated name as String
    generated_name.to_string()
}
