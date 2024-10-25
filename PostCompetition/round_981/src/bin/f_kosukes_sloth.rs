use std::io::BufRead;

const BIG_MODULUS: u64 = 1_000_000_007;

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
        let line = line_iter.next().unwrap().unwrap();
        let (n_str, k_str) = line.split_once(' ').unwrap();
        let n = n_str.parse::<u64>().unwrap() % BIG_MODULUS;
        let k = k_str.parse::<u64>().unwrap();

        let mut prev: u64 = 0;
        let mut fib: u64 = 1 % k;
        let mut period: u64 = 1;

        // Calculate the fibonacci numbers, modulus k (since Fib and % are additive),
        // until a zero is encountered. This period repeats, so we can simply multiply it by n.
        while fib != 0 {
            (prev, fib) = (fib, (prev + fib) % k);
            period += 1;
        }
        let answer = (n * period) % BIG_MODULUS;
        println!("{answer}");
    }
}
