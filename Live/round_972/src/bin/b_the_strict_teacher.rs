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
        let n_m_q_str = line_iter.next().unwrap().unwrap();
        let mut n_m_q_iter = n_m_q_str.split_whitespace();
        let n: usize = n_m_q_iter.next().unwrap().parse().unwrap();
        // Don't need the following:
        // let m: usize = n_m_q_iter.next().unwrap().parse().unwrap();
        // let q: usize = n_m_q_iter.next().unwrap().parse().unwrap();
        let mut bs: Vec<usize> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|b| b.parse().unwrap())
            .collect();
        bs.sort();
        let min_b = *bs.first().unwrap();
        let max_b = *bs.last().unwrap();
        let b_ranges: Vec<(usize, usize)> = bs
            .windows(2)
            .filter_map(|w| {
                let (s, b) = (w[0], w[1]);
                if s == b {
                    None
                } else {
                    Some((s, b))
                }
            }).collect::<Vec<_>>();
        
        let q_answers: Vec<String> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|q_str| {
                let q: usize = q_str.parse().unwrap();
                let steps: usize = if q < min_b {
                    min_b - 1
                } else if q > max_b {
                    n - max_b
                } else {
                    // Find the pair of numbers that sandwich q using a binary search:
                    let b_range: (usize, usize) = find_containing_range(q, &b_ranges);
                    (b_range.1 - b_range.0) / 2
                };
                steps.to_string()
            })
            .collect();
        let answer = q_answers.join("\n");
        println!("{answer}");
    }
}

fn find_containing_range(q: usize, b_ranges: &[(usize, usize)]) -> (usize, usize) {
    let mut min_index = 0;
    let mut max_index = b_ranges.len() - 1;
    while min_index < max_index {
        let mid_index = (min_index + max_index) / 2;
        if b_ranges[mid_index].0 < q {
            // Check if this is the right one...
            if b_ranges[mid_index].1 > q {
                return b_ranges[mid_index]
            }
            min_index = mid_index + 1;
        } else {
            max_index = mid_index - 1;
        }
    }
    b_ranges[min_index]
}
