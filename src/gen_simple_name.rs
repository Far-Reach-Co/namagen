use rand::{random, seq::SliceRandom};

pub fn gen_simple_name(
    consonant_inventory: Vec<&str>,
    vowel_inventory: Vec<&str>,
    syllables_per_name: i32,
) -> String {
    let mut rng = rand::thread_rng();
    let mut most_recent_grapheme = "";
    let mut generated_syllables = 0;
    let mut two_vowels_or_consonants_in_a_row = false;
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
        let random_vowel: String = vowel_inventory.choose(&mut rng).unwrap().to_string();
        generated_name.push_str(&random_vowel);
        generated_syllables += 1;
        most_recent_grapheme = "vowel";
    }

    // Add letters until syllable count is reached
    while generated_syllables < syllables_per_name {
        if two_vowels_or_consonants_in_a_row == true {
            if most_recent_grapheme == "consonant" {
                let random_vowel: String = vowel_inventory.choose(&mut rng).unwrap().to_string();
                generated_name.push_str(&random_vowel);
                generated_syllables += 1;
                most_recent_grapheme = "vowel";
                two_vowels_or_consonants_in_a_row = false;
            } else {
                let random_consonant: String =
                    consonant_inventory.choose(&mut rng).unwrap().to_string();
                generated_name.push_str(&random_consonant);
                most_recent_grapheme = "consonant";
                two_vowels_or_consonants_in_a_row = false;
            }
        } else {
            if random() {
                let random_vowel: String = vowel_inventory.choose(&mut rng).unwrap().to_string();
                generated_name.push_str(&random_vowel);
                if most_recent_grapheme == "consonant" {
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                } else {
                    two_vowels_or_consonants_in_a_row = true
                }
            } else {
                let random_consonant: String =
                    consonant_inventory.choose(&mut rng).unwrap().to_string();
                generated_name.push_str(&random_consonant);
                if most_recent_grapheme == "consonant" {
                    two_vowels_or_consonants_in_a_row = true
                } else {
                    most_recent_grapheme = "consonant";
                }
            }
        }
    }

    generated_name.to_string()
}
