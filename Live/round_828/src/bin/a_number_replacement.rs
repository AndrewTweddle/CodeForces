use std::collections::HashMap;
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
        let n = line_iter.next().unwrap().unwrap().parse::<usize>().unwrap();
        let nums_str = line_iter.next().unwrap().unwrap();
        let nums: Vec<u8> = nums_str
            .split(' ')
            .map(|num_str| num_str.parse::<u8>().unwrap())
            .collect();
        let str_to_match = line_iter.next().unwrap().unwrap();

        let mut num_to_char_map: HashMap<u8, char> = HashMap::with_capacity(n);
        let mut is_consistent = true;

        for (ch, num) in str_to_match.chars().zip(nums) {
            if let Some(&prev_ch) = num_to_char_map.get(&num) {
                if prev_ch != ch {
                    is_consistent = false;
                    break;
                }
            } else {
                num_to_char_map.insert(num, ch);
            }
        }

        let output = if is_consistent { "YES" } else { "NO" };
        println!("{}", output);
    }
}
