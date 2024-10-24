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
        let n: usize = line_iter.next().unwrap().unwrap().parse::<usize>().unwrap();
        let diag_count = 2 * n + 1;
        let mut min_neg = vec![0_i32; diag_count];
        for i in 0..n {
            line_iter
                .next()
                .unwrap()
                .unwrap()
                .split_ascii_whitespace()
                .enumerate()
                .for_each(|(j, num_str)| {
                    let num: i32 = num_str.parse().unwrap();
                    if num < 0 {
                        let diag = n + j - i - 1;
                        if min_neg[diag] > num {
                            min_neg[diag] = num;
                        }
                    }
                })
        }
        let min_neg_sum = -min_neg.iter().sum::<i32>();
        println!("{min_neg_sum}");
    }
}
