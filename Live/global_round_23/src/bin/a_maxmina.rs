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
        let (_n_str, _k_str) = line_iter.next().unwrap().unwrap().split_once(' ').unwrap();
        // not needed... let (_n, _k) = (n_str.parse::<u8>(), k_str.parse::<u8>());
        let any_non_zero = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .any(|digit| digit == "1");
        let output = if any_non_zero { "YES" } else { "NO" };
        println!("{}", output);
    }
}
