//! Formatting to string logic
use crate::config::Config;

/// Converts an unsigned number to a vector of its digits.
fn number_to_vec(n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

/// Converts a single digit number to a Discord style emoji.
pub fn num_to_emoji(num: u32) -> String {
    let mut output = String::new();
    for num in number_to_vec(num) {
        let emoji = match num {
            0 => ":zero:",
            1 => ":one:",
            2 => ":two:",
            3 => ":three:",
            4 => ":four:",
            5 => ":five:",
            6 => ":six:",
            7 => ":seven:",
            8 => ":eight:",
            9 => ":nine:",
            _ => "",
        };
        output.push_str(emoji)
    }
    output
}

/// Returns a vector of strings that are padded to the same length.
///
/// Also takes a minimum length to pad to.
pub fn answers_padded(config: &Config, min_len: i32) -> Vec<String> {
    let answer_strings: Vec<String> = config
        .answers
        .iter()
        .map(|a| {
            format!(
                "{}{}",
                a.operation,
                match a.number {
                    Some(v) => format!(" {}", v.to_string()),
                    None => "".into(),
                }
            )
        })
        .collect();

    let max_length_string = answer_strings
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    let answers_padded: Vec<String> = answer_strings
        .iter()
        .map(|string| {
            let mut out = format!("{}", string);
            let padding = std::cmp::max(max_length_string, min_len as usize) - string.len();
            for _ in 0..padding {
                out.push_str(" ");
            }
            out
        })
        .collect();

    answers_padded
}
