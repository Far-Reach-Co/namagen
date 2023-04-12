pub fn trans_to_basic_latin<'a>(
    word_to_transliterate: &'a str,
    language: &'a str,
    alphabet: Vec<(&'static str, &'static str, i32)>,
) -> &'a str {
    let chars: Vec<&str> = word_to_transliterate.split("").collect();
    for letter in alphabet {
        if letter.0 != letter.1 {
            // Replace each instance of that letter[0] in cars with letter[1]
        }
    }
    // if language == "Saurian" && {one or more of the letters ƹ, h, ṭ, ṣ, q, r, ḍ, and ẓ are in chars} {
    if language == "Saurian" {
        // Replace each instance of the letter 'e' in chars with the letter 'a'
    }
    chars
}
