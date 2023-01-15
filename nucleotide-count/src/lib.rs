use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid_chars = ['A', 'C', 'G', 'T'];
    if !valid_chars.contains(&nucleotide) {
        return Err(nucleotide);
    }
    let mut count = 0;
    for each_char in dna.chars() {
        if !valid_chars.contains(&each_char) {
            return Err(each_char);
        }
        if each_char == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut list_of_nucleotide: HashMap<char, usize> =
        HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    let valid_chars = ['A', 'C', 'G', 'T'];

    for each_char in dna.chars() {
        if !valid_chars.contains(&each_char) {
            return Err(each_char);
        }
        list_of_nucleotide
            .entry(each_char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    Ok(list_of_nucleotide)
}
