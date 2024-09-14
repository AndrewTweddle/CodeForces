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
        let n_str = line_iter.next().unwrap().unwrap();
        let n: usize = n_str.parse().unwrap();
        let min_num_of_each = n / 5;
        let count_with_one_more = n % 5;
        let answer: String = ["a", "e", "i", "o", "u"]
            .iter()
            .enumerate()
            .map(|(i, vowel)| {
                let repetitions = if i < count_with_one_more {
                    min_num_of_each + 1
                } else {
                    min_num_of_each
                };
                vowel.repeat(repetitions)
            })
            .collect();
        println!("{answer}");
    }
}
