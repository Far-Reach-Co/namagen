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
    let syllables_per_name = rng.gen_range(1..4);
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
                if random() && random() {
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
        } else if random() && random() {
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
    if most_recent_grapheme != "vowel" {
    } else {
        if random() && random() {
            generated_name.push_str(
                &vowel_and_syllabic_inventory
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
            );
        } else {
            generated_name.push_str(&vowel_inventory.choose(&mut rng).unwrap().to_string());
        }
        most_recent_grapheme = "vowel";
        generated_syllables += 1;
    }

    // Add letters until syllable count is reached
    while generated_syllables < syllables_per_name && generated_name.chars().count() < 8 {
        if two_vowels_or_consonants_in_a_row == false {
            if random() {
                generated_name.push_str(&consonant_inventory.choose(&mut rng).unwrap().to_string());
                if most_recent_grapheme == "consonant" {
                    two_vowels_or_consonants_in_a_row = true
                } else {
                    most_recent_grapheme = "consonant";
                }
            } else if random() && random() {
                generated_name.push_str(
                    &vowel_and_syllabic_inventory
                        .choose(&mut rng)
                        .unwrap()
                        .to_string(),
                );
                if most_recent_grapheme == "consonant" {
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                } else {
                    two_vowels_or_consonants_in_a_row = true
                }
            } else {
                generated_name.push_str(&vowel_inventory.choose(&mut rng).unwrap().to_string());
                if most_recent_grapheme == "consonant" {
                    generated_syllables += 1;
                    most_recent_grapheme = "vowel";
                } else {
                    two_vowels_or_consonants_in_a_row = true
                }
            }
        } else {
            if most_recent_grapheme == "consonant" {
                if random() && random() {
                    generated_name.push_str(
                        &vowel_and_syllabic_inventory
                            .choose(&mut rng)
                            .unwrap()
                            .to_string(),
                    );
                } else {
                    generated_name.push_str(&vowel_inventory.choose(&mut rng).unwrap().to_string());
                }
                generated_syllables += 1;
                most_recent_grapheme = "vowel";
            } else {
                generated_name.push_str(&consonant_inventory.choose(&mut rng).unwrap().to_string());
                most_recent_grapheme = "consonant";
            }
            two_vowels_or_consonants_in_a_row = false;
        }
    }

    // Potentially add a final grapheme or two
    if random() {
        generated_name.push_str(&consonant_inventory.choose(&mut rng).unwrap().to_string());
        most_recent_grapheme = "consonant";
    } else if random() && random() {
        generated_name.push_str(
            &vowel_and_syllabic_inventory
                .choose(&mut rng)
                .unwrap()
                .to_string(),
        );
    } else {
        generated_name.push_str(&vowel_inventory.choose(&mut rng).unwrap().to_string());
    }

    if most_recent_grapheme == "vowel" && random() {
        generated_name.push_str(&consonant_inventory.choose(&mut rng).unwrap().to_string());
    }

    // Return generated name as String
    generated_name.to_string()
}
