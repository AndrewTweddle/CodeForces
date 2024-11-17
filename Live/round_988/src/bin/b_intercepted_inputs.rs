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
        let n_times_m = line_iter.next().unwrap().unwrap().parse::<u64>().unwrap() - 2;
        let mut div_pairs_to_find: HashMap<u64, u64> = HashMap::with_capacity(n_times_m as usize);

        let nums_str = line_iter.next().unwrap().unwrap();
        let nums_iter = nums_str
            .split(' ')
            .map(|num_str| num_str.parse::<u64>().unwrap());
        let (n, m) = nums_iter
            .filter_map(|n| {
                if let Some(&m) = div_pairs_to_find.get(&n) {
                    Some((n, m))
                } else {
                    if n != 0 && (n_times_m % n == 0) {
                        let m = n_times_m / n;
                        div_pairs_to_find.insert(m, n);
                    }
                    None
                }
            })
            .next()
            .unwrap();
        println!("{n} {m}");
    }
}
