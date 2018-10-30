pub mod piglatin {

    #[derive(PartialEq, Debug)]
    pub enum LetterType {
        CONSONANT,
        VOWEL,
    }

    pub fn translate(input: &str) -> String {
        let mut translated_output = String::from("");

        for word in input.split_whitespace() {
            let converted_word = convert_word(word);

            translated_output += &format!("{} ", converted_word);
        }

        String::from(translated_output.trim())
    }

    pub fn convert_word(input: &str) -> String {
        if get_first_letter_type(input) == LetterType::CONSONANT {
            convert_consonant_word(input)
        } else {
            convert_vowel_word(input)
        }
    }

    pub fn get_first_letter_type(input: &str) -> LetterType {
        let first_letter = get_first_letter(input);

        if first_letter != 'a'
            && first_letter != 'e'
            && first_letter != 'i'
            && first_letter != 'o'
            && first_letter != 'u'
        {
            LetterType::CONSONANT
        } else {
            LetterType::VOWEL
        }
    }

    pub fn convert_consonant_word(input: &str) -> String {
        let first_letter = get_first_letter(input);

        let mut is_first = true;
        let mut output = String::from("");

        for letter in input.chars() {
            if is_first {
                is_first = false;
                continue;
            }

            output.push(letter);
        }

        format!("{}-{}ay", output, first_letter)
    }

    pub fn convert_vowel_word(input: &str) -> String {
        format!("{}-hay", input)
    }

    fn get_first_letter(input: &str) -> char {
        input.chars().next().expect("Must be first letter")
    }
}

#[cfg(test)]
mod tests {
    use super::piglatin;

    #[test]
    fn first_letter_type() {
        assert_eq!(
            piglatin::get_first_letter_type("dog"),
            piglatin::LetterType::CONSONANT
        );
        assert_eq!(
            piglatin::get_first_letter_type("enum"),
            piglatin::LetterType::VOWEL
        );
    }

    #[test]
    fn convert_consonant_word() {
        assert_eq!(piglatin::convert_consonant_word("dog"), "og-day");
    }

    #[test]
    fn convert_vowel_word() {
        assert_eq!(piglatin::convert_vowel_word("apples"), "apples-hay");
    }

    #[test]
    fn convert_word() {
        assert_eq!(piglatin::convert_word("leaves"), "eaves-lay");
        assert_eq!(piglatin::convert_word("orchard"), "orchard-hay");
    }

    #[test]
    fn translate() {
        assert_eq!(
            piglatin::translate("away at night in the sea"),
            "away-hay at-hay ight-nay in-hay he-tay ea-say"
        );
    }
}
