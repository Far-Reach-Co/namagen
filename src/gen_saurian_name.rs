use rand::Rng;
use rand::{random, seq::SliceRandom, thread_rng};

use crate::get_saurian_alphabet::{
    get_saurian_consonsants, get_saurian_true_vowels, get_saurian_vowels_and_syllabics,
};

fn assemble_grapheme_inventory<'a>(letters: Vec<(&'a str, &'a str, i32)>) -> Vec<&'a str> {
    let mut grapheme_inventory = vec![""];
    for letter in letters {
        grapheme_inventory.push(letter.0)
    }
    grapheme_inventory
}
pub fn gen_saurian_name() -> String {
    let consonant_inventory = assemble_grapheme_inventory(get_saurian_consonsants());
    let vowel_and_syllabic_inventory =
        assemble_grapheme_inventory(get_saurian_vowels_and_syllabics());
    let vowel_inventory = assemble_grapheme_inventory(get_saurian_true_vowels());
    let mut rng = thread_rng();
    let mut max_chars = 8;
    let syllables_per_name = if random() && random() {
        2
    } else {
        if random() {
            max_chars = rng.gen_range(4..8)
        }
        rng.gen_range(1..4)
    };
    let mut most_recent_grapheme = "";
    let mut two_vowels_or_consonants_in_a_row = false;
    let mut generated_syllables = 0;
    let mut generated_name = most_recent_grapheme.to_string();

    // While loop is a temporary workaround for missing an uppercase letter at start of word
    // Maybe this will be fully fixed when I rewrite this using recursion
    while generated_name == "" {
        // Generate first letter
        if random() {
            generated_name.push_str(
                &consonant_inventory
                    .choose(&mut rng)
                    .unwrap()
                    .to_string()
                    .to_uppercase(),
            );
            most_recent_grapheme = "consonant";

            if generated_name == "'" {
                if rng.gen_range(1..8) == 8 {
                    generated_name.push_str(
                        &vowel_and_syllabic_inventory
                            .choose(&mut rng)
                            .unwrap()
                            .to_string()
                            .to_uppercase(),
                    );
                } else {
                    generated_name = vowel_inventory
                        .choose(&mut rng)
                        .unwrap()
                        .to_string()
                        .to_uppercase();
                }
                most_recent_grapheme = "vowel";
            };
        } else if rng.gen_range(1..8) == 8 {
            generated_name.push_str(
                &vowel_and_syllabic_inventory
                    .choose(&mut rng)
                    .unwrap()
                    .to_string()
                    .to_uppercase(),
            );
            most_recent_grapheme = "vowel";
        } else {
            generated_name.push_str(
                &vowel_inventory
                    .choose(&mut rng)
                    .unwrap()
                    .to_string()
                    .to_uppercase(),
            );
            most_recent_grapheme = "vowel";
        }
    }

    // If first letter is a consonant, add second letter as a vowel (so we don't have names that start with two consonants like "Tdeneb")
    // Result exploit is a temporary workaround
    if most_recent_grapheme != "vowel" {
        let mut result = "".to_string();
        if result != "" {
            return "What??".to_string();
        }
        if rng.gen_range(1..4) == 4 {
            result = vowel_and_syllabic_inventory
                .choose(&mut rng)
                .unwrap()
                .to_string();
        } else {
            result = vowel_inventory.choose(&mut rng).unwrap().to_string();
        }
        if result == "" {
            generated_name.push_str("a")
        } else {
            generated_name.push_str(&result)
        }
        most_recent_grapheme = "vowel";
        generated_syllables += 1;
    }

    // Add letters until syllable count is reached
    // Result exploit is a temporary workaround
    while generated_syllables < syllables_per_name && generated_name.chars().count() < max_chars {
        let mut result = "".to_string();
        if result != "" {
            return "What??".to_string();
        }
        if two_vowels_or_consonants_in_a_row == true {
            if most_recent_grapheme == "consonant" {
                if rng.gen_range(1..8) == 8 {
                    result = vowel_and_syllabic_inventory
                        .choose(&mut rng)
                        .unwrap()
                        .to_string();
                } else {
                    result = vowel_inventory.choose(&mut rng).unwrap().to_string();
                }
                if result == "" {
                    result = "i".to_string();
                }
                generated_syllables += 1;
                most_recent_grapheme = "vowel";
            } else {
                result = consonant_inventory.choose(&mut rng).unwrap().to_string();
                if result == "" {
                    result = "k".to_string()
                }
                most_recent_grapheme = "consonant";
            }
            two_vowels_or_consonants_in_a_row = false;
        } else {
            if random() {
                result = consonant_inventory.choose(&mut rng).unwrap().to_string();
                if most_recent_grapheme == "consonant" {
                    two_vowels_or_consonants_in_a_row = true
                } else {
                    most_recent_grapheme = "consonant";
                }
                if result == "" {
                    result = "n".to_string()
                }
            } else if rng.gen_range(1..6) == 5 {
                result = vowel_and_syllabic_inventory
                    .choose(&mut rng)
                    .unwrap()
                    .to_string();
                if most_recent_grapheme == "consonant" {
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                } else {
                    two_vowels_or_consonants_in_a_row = true
                }
                if result == "" {
                    result = "l̩".to_string()
                }
            } else {
                result = vowel_inventory.choose(&mut rng).unwrap().to_string();
                if most_recent_grapheme == "consonant" {
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                } else {
                    two_vowels_or_consonants_in_a_row = true
                }
                if result == "" {
                    result = "a".to_string()
                }
            }
        }
        generated_name.push_str(&result);
    }

    // Potentially add a final grapheme or two
    if random() && random() {
        generated_name.push_str(&consonant_inventory.choose(&mut rng).unwrap().to_string());
        most_recent_grapheme = "consonant";
    } else if rng.gen_range(1..8) == 8 {
        generated_name.push_str(
            &vowel_and_syllabic_inventory
                .choose(&mut rng)
                .unwrap()
                .to_string(),
        );
    } else {
        generated_name.push_str(&vowel_inventory.choose(&mut rng).unwrap().to_string());
    }

    if most_recent_grapheme == "vowel" && random() && random() {
        generated_name.push_str(&consonant_inventory.choose(&mut rng).unwrap().to_string());
    }

    // Return generated name as String
    generated_name.to_string()
}
