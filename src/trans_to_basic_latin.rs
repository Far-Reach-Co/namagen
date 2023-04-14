pub fn trans_to_basic_latin<'a>(
    word_to_transliterate: &'a str,
    language: &'a str,
    alphabet: Vec<(&'static str, &'static str, i32)>,
) -> String {
    let mut word_basic_latin = word_to_transliterate.to_owned();

    // Convert fist letter to lowercase
    let mut chars = word_basic_latin.chars();
    word_basic_latin = match chars.next() {
        None => String::new(),
        Some(char) => char.to_lowercase().collect::<String>() + chars.as_str(),
    };
    let word_to_transliterate_lowercase = word_basic_latin.to_owned();

    for letter in alphabet {
        if letter.0 != letter.1 {
            word_basic_latin = word_basic_latin.replace(letter.0, letter.1)
        }
    }
    word_basic_latin = word_basic_latin.replace("̩̩", "");
    word_basic_latin = word_basic_latin.replace("̩", "");
    // lazy
    if language == "Saurian"
        && word_to_transliterate_lowercase.find(|c: char| {
            c == 'ɛ'
                || c == 'h'
                || c == 'ṭ'
                || c == 'ṣ'
                || c == 'q'
                || c == 'r'
                || c == 'ḍ'
                || c == 'ẓ'
        }) != None
    {
        word_basic_latin = word_basic_latin.replace("e", "a")
    }

    // Convert fist letter to uppercase
    chars = word_basic_latin.chars();
    word_basic_latin = match chars.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
    };

    word_basic_latin
}
