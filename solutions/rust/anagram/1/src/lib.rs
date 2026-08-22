use std::collections::HashSet;

pub fn get_freq(lower_word: &str) -> (String, Vec<usize>) {
    let mut freq = vec![0; 26];

    for byte in lower_word.bytes() {
        if byte.is_ascii_alphabetic() {
            let idx = (byte - b'a') as usize;
            freq[idx] += 1
        }
    }

    (lower_word.to_string(), freq)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_ascii_lowercase();
    let word_freq = get_freq(&word_lower);

    possible_anagrams
        .iter()
        .copied()
        .filter(|&cand| {
            let cand_lower = cand.to_lowercase();
            cand_lower != word_freq.0 && get_freq(&cand_lower).1 == word_freq.1
        })
        .collect()
}
