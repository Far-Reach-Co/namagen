pub fn trans_to_basic_latin<'a>(
    word_to_transliterate: &'a str,
    language: &'a str,
    alphabet: Vec<(&'static str, &'static str, i32)>,
) -> String {
    let mut word_basic_latin = word_to_transliterate.to_owned();
    for letter in alphabet {
        if letter.0 != letter.1 {
            word_basic_latin = word_basic_latin.replace(letter.0, letter.1)
        }
    }
    word_basic_latin = word_basic_latin.replace("̩̩", "");
    // lazy
    if language == "Saurian"
        && word_to_transliterate.find(|c: char| {
            c == 'ƹ'
                || c == 'h'
                || c == 'ṭ'
                || c == 'ṣ'
                || c == 'q'
                || c == 'r'
                || c == 'r'
                || c == 'ḍ'
                || c == 'ẓ'
        }) != None
    {
        word_basic_latin = word_basic_latin.replace("e", "a")
    }
    word_basic_latin
}
