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
        let mut topics: Vec<u32> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect();
        for i in 0..n {
            let j = n - i - 1;

            // Consider whether to swap indices i and j
            let this = topics[i];
            let that = topics[j];
            if this == that {
                continue;
            }

            let mut dist_change: i8 = 0;

            // Count the number of times a disturbance is reduced
            if i > 0 && topics[i - 1] == this {
                dist_change -= 1;
            }
            if i < n - 1 && topics[i + 1] == this {
                dist_change -= 1;
            }
            if j > 0 && topics[j - 1] == that {
                dist_change -= 1;
            }
            if j < n - 1 && topics[j + 1] == that {
                dist_change -= 1;
            }

            // Count the number of times a disturbance is created
            if i > 0 && topics[i - 1] == that {
                dist_change += 1;
            }
            if i < n - 1 && topics[i + 1] == that {
                dist_change += 1;
            }
            if j > 0 && topics[j - 1] == this {
                dist_change += 1;
            }
            if j < n - 1 && topics[j + 1] == this {
                dist_change += 1;
            }

            if dist_change < 0 {
                topics[i] = that;
                topics[j] = this;
            }
        }
        let total_disturbance = topics.windows(2).filter(|w| w[0] == w[1]).count();
        println!("{total_disturbance}");
    }
}
