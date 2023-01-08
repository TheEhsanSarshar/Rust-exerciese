use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {

    let inputs_chars = UnicodeSegmentation::graphemes(input, true).collect()::<Vec<&str>>();
    let mut reversed_chars: Vec<char> = vec!['0'; inputs_chars.count()];
    let inputs_chars = input.chars();
    let reversed_chars_length = reversed_chars.len();

    for (index, ch) in inputs_chars.enumerate() {
        reversed_chars[reversed_chars_length - 1 - index] = ch;
    }

    reversed_chars.into_iter().collect()
}
