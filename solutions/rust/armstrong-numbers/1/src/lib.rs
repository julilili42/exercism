pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let num_str = num.to_string();
    let n = num_str.len() as u32;

    let total: u64 = num_str
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(|digit| (digit as u64).pow(n))
        .sum();

    total == num as u64
}

pub fn is_armstrong_number_bad(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let num_str = format!("{}", num);
    let n = num_str.len();

    let mut total: u64 = 0;
    for ch in num_str.chars() {
        if let Some(number) = ch.to_digit(10) {
            total += (number as u64).pow(n as u32);
        }
    }
    total == num as u64
}
