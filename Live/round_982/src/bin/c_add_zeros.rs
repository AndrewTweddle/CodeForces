use std::collections::HashMap;
use std::io::BufRead;

type Memo = HashMap<usize, usize>;

type Lookup = HashMap<usize, Vec<usize>>;

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
        let mut lookup: Lookup = Lookup::new();

        line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .enumerate()
            .skip(1) // The first entry adds 0 zeros, so ignore it
            .for_each(|(i, num)| {
                if i + num >= n {
                    lookup.entry(i + num - n).or_default().push(i);
                }
            });

        let mut memo: Memo = Memo::new();
        let max_length = n + calculate_max_zeros(0, &lookup, &mut memo);
        println!("{max_length}");
    }
}

fn calculate_max_zeros(num_zeros_appended: usize, lookup: &Lookup, memo: &mut Memo) -> usize {
    if let Some(max_zeros) = memo.get(&num_zeros_appended) {
        *max_zeros
    } else {
        let max_zeros = if let Some(candidates) = lookup.get(&num_zeros_appended) {
            candidates
                .iter()
                .map(|&num_new_zeros| {
                    calculate_max_zeros(num_zeros_appended + num_new_zeros, lookup, memo)
                })
                .max()
                .unwrap()
        } else {
            num_zeros_appended
        };

        memo.insert(num_zeros_appended, max_zeros);
        max_zeros
    }
}
