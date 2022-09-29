use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let mut line_iter = stdin_lock.lines();
    let t = line_iter
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    for _ in 0..t {
        // read string s and convert its UTF-8 digits to bytes
        let s = line_iter.next().unwrap().unwrap();
        let digits: Vec<u8> = s.chars().map(|ch| ch as u8 - b'0').collect();
        let output = sort_digits(&digits);

        println!("{}", output);
    }
}

fn sort_digits(digits: &[u8]) -> String {
    let mut digit_counts = [0_usize; 10];
    let mut start_pos: usize = 0;

    for target_digit in 0_u8..10 {
        if let Some(end_pos_offset) = digits[start_pos..]
            .iter()
            .rposition(|&digit| digit == target_digit)
        {
            let end_pos = start_pos + end_pos_offset;
            for pos in start_pos..=end_pos {
                let mut digit = digits[pos];
                if digit != target_digit && digit != 9 {
                    digit += 1;
                }
                digit_counts[digit as usize] += 1;
            }
            start_pos = end_pos + 1;
            if start_pos >= digits.len() {
                break;
            }
        }
    }

    let mut sorted_digits = vec![0_u8; digits.len()];
    let mut cum_digit_count: usize = 0;
    for (digit, &digit_count) in digit_counts.iter().enumerate() {
        if digit_count == 0 {
            continue;
        }
        let utf8_digit = b'0' + digit as u8;
        let next_cum_digit_count = cum_digit_count + digit_count;
        sorted_digits[cum_digit_count..next_cum_digit_count].fill(utf8_digit);
        cum_digit_count = next_cum_digit_count;
    }

    unsafe { String::from_utf8_unchecked(sorted_digits) }
}
