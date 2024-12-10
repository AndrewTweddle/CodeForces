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
    line_iter.take(t).for_each(|n_str| {
        let n = n_str.unwrap().parse::<usize>().unwrap();
        let answer = solve(n);
        println!("{}", answer);
    });
}

fn solve(mut n: usize) -> usize {
    match n {
        1 => 1,
        2..=4 => 2,
        _ => {
            let mut i = 2;
            while n > 4 {
                n = (n - 1) / 2;
                i += 1;
            }
            i
        }
    }
}