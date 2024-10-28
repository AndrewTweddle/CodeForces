use std::cmp::Ordering;
use std::io::BufRead;

const MODULUS: usize = 1_000_000_007;

#[derive(Copy, Clone)]
struct Solution {
    min_cost: usize,
    num_op_sequences: usize,
}

impl Solution {
    fn new(min_cost: usize, num_op_sequences: usize) -> Self {
        Self {
            min_cost,
            num_op_sequences,
        }
    }
}

// Track the minimum cost by the number of b values left (which determines k)
// and the number of a values left.
// If the vector entry is None, the value has not yet been saved.
// If the entry's value is None, then there was no solution.
type Memo = Vec<Vec<Option<Option<Solution>>>>;

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
        let (n_str, m_str) = line.split_once(' ').unwrap();
        let n = n_str.parse::<usize>().unwrap();
        let m = m_str.parse::<usize>().unwrap();

        let a_vals: Vec<usize> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();

        let b_vals: Vec<usize> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();

        let mut memo: Memo = vec![vec![None; n + 1]; m + 1];

        // When there are no more a's left, then success has been achieved.
        for bs_left_count in 0..(m + 1) {
            memo[bs_left_count][0] = Some(Some(Solution::new(0, 1)));
        }

        // When there are no more b's left, but there are still a's, then no solution was found:
        for as_left_count in 1..(n + 1) {
            memo[0][as_left_count] = Some(None);
        }

        let solution = calculate_solution(&a_vals, &b_vals, &mut memo);
        if let Some(solution) = solution {
            println!("{} {}", solution.min_cost, solution.num_op_sequences);
        } else {
            println!("-1");
        }
    }
}

fn calculate_solution(
    as_left: &[usize],
    bs_left: &[usize],
    memo: &mut Memo
) -> Option<Solution> {
    let as_left_count = as_left.len();
    let bs_left_count = bs_left.len();

    if let Some(cached_solution) = memo[bs_left_count][as_left_count].clone() {
        cached_solution
    } else {
        // Option 1: Increase k by 1
        let mut maybe_best_solution = calculate_solution(as_left, &bs_left[1..], memo);

        // Option 2: Remove a prefix of r of the a's, for each valid r
        let next_b = bs_left[0];
        let mut cum_sum = 0;

        for r in 1..=as_left_count {
            cum_sum += as_left[r - 1];
            if cum_sum > next_b {
                break;
            }

            let mut prefix_removal_solution = calculate_solution(&as_left[r..], bs_left, memo);

            if let Some(ref mut rem_soln) = prefix_removal_solution {
                rem_soln.min_cost += bs_left_count - 1;

                if let Some(ref mut best_solution) = maybe_best_solution {
                    match best_solution.min_cost.cmp(&rem_soln.min_cost) {
                        Ordering::Less => {}
                        Ordering::Equal => {
                            best_solution.num_op_sequences += rem_soln.num_op_sequences;
                            best_solution.num_op_sequences %= MODULUS;
                        }
                        Ordering::Greater => {
                            maybe_best_solution = prefix_removal_solution;
                        }
                    }
                } else {
                    maybe_best_solution = prefix_removal_solution;
                }
            }
        }

        memo[bs_left_count][as_left_count] = Some(maybe_best_solution);
        maybe_best_solution
    }
}
