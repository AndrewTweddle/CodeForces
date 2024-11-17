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
        let params_str = line_iter.next().unwrap().unwrap();
        let params: Vec<i64> = params_str
            .split(' ')
            .map(|num_str| num_str.parse::<i64>().unwrap())
            .collect();
        let n = params[0];
        let b = params[1];
        let c = params[2];
        let answer = match (n, b, c) {
            (1, 0, 0) => 0,
            (_, 0, c) if c < n - 2 => -1,
            (_, 0, c) if c >= n - 2 => n,
            (_, 0, c) if c == n - 1 => n - 1,
            (_, 0, _) => n,
            (_, 1, 0) => 0,
            _ => n - 1 - (n - c) / b,
        };
        println!("{answer}");
    }
}
