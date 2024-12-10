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
        let n_k_str = line_iter.next().unwrap().unwrap();
        let (n_str, k_str) = n_k_str.split_once(' ').unwrap();
        let _n: usize = n_str.parse().unwrap();
        let k: i8 = k_str.parse().unwrap();
        let numbers: Vec<i8> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if let Some(index) = numbers.iter().enumerate().position(|(i, num)| {
            numbers.iter().enumerate().all(|(j, other_num)| {
                (i == j) || (other_num - num).abs() % k != 0
            })
        }) {
            println!("YES");
            println!("{}", index + 1);
        } else {
            println!("NO");
        }
    }
}
