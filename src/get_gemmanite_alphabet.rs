pub fn get_gemmanite_nasals() -> Vec<(&'static str, &'static str, i32)> {
    // Tuples are of the form (default_grapheme, basic_grapheme, frequency_of_occurrance)
    vec![
        ("XX", "ng", 12),
        ("XX", "nc?", 12),
        ("n", "n", 12),
        ("m", "m", 12),
    ]
}

pub fn get_gemmanite_consonsants() -> Vec<(&'static str, &'static str, i32)> {
    // Tuples are of the form (default_grapheme, basic_grapheme, frequency_of_occurrance)
    vec![
        ("'", "'", 12),
        ("j", "j", 12),
        ("d", "d", 12),
        ("h", "h", 12),
        ("w", "v", 12),
        ("z", "z", 12),
        ("ħ", "h", 12),
        ("ṭ", "t", 12),
        ("y", "y", 12),
        ("k", "k", 12),
        ("l", "l", 12),
        ("n", "n", 12),
        ("s", "s", 12),
        ("ƹ", "aa", 12),
        ("f", "f", 12),
        ("ṣ", "s", 12),
        ("q", "q", 12),
        ("r", "r", 12),
        ("š", "sh", 12),
        ("t", "t", 12),
        ("θ", "th", 12),
        ("x", "kh", 12),
        ("ð", "th", 12),
        ("ḍ", "d", 12),
        ("ẓ", "z", 12),
        ("ɣ", "gh", 12),
    ]
}
