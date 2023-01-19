use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut hashmap_of_words: HashMap<String, u32> = HashMap::new();
    let mut the_word = String::new();
    let length = words.len();
    for (index, each_char) in words.chars().enumerate() {
        if each_char.is_ascii_alphanumeric() || each_char == '\'' {
            the_word.push(each_char);
            if index == length - 1 {
                if !the_word.is_empty() {
                    if the_word.starts_with('\'') {
                        the_word.remove(0);
                    }
                    if the_word.ends_with('\'') {
                        the_word.remove(the_word.len() - 1);
                    }
                    let entry = hashmap_of_words.entry(the_word.to_lowercase()).or_default();
                    *entry += 1;
                    the_word = String::new()
                }
            }
        } else {
            if !the_word.is_empty() {
                if the_word.starts_with('\'') {
                    the_word.remove(0);
                }
                if the_word.ends_with('\'') {
                    the_word.remove(the_word.len() - 1);
                }
                let entry = hashmap_of_words.entry(the_word.to_lowercase()).or_default();
                *entry += 1;
                the_word = String::new()
            }
        }
    }
    hashmap_of_words
}
