pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
pub fn reverse_bad(input: &str) -> String {
    let mut res = String::new();
    for ch in input.chars().rev() {
        res.push(ch);
    }
    res
}
