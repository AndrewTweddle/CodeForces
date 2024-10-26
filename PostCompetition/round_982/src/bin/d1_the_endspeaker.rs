use std::io::BufRead;

// Track the minimum cost by the value of k and the number of a values left
// If the vector entry is None, the value has not yet been saved.
// If the entry's value is None, then there was no solution.
type Memo = Vec<Vec<Option<Option<usize>>>>;

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
        
        let mut memo: Memo = vec![vec![None; n+1]; m+1];
        
        // When there are no more a's left, then success has been achieved:
        for bs_left_count in 0..(m+1) {
            memo[bs_left_count][0] = Some(Some(0));
        }
        
        // When there are no more b's left, but there are still a's, then no solution was found:
        for as_left_count in 1..(n+1) {
            memo[0][as_left_count] = Some(None);
        }
        
        let min_cost = calculate_min_cost(&a_vals, &b_vals, &mut memo);
        if let Some(min_cost) = min_cost {
            println!("{min_cost}");
        } else {
            println!("-1");
        }
    }
}

fn calculate_min_cost(as_left: &[usize], bs_left: &[usize], memo: &mut Memo) -> Option<usize> {
    let as_left_count = as_left.len();
    let bs_left_count = bs_left.len();
    
    if let Some(maybe_min_cost) = memo[bs_left_count][as_left_count] {
        maybe_min_cost
    } else {
        let min_cost = {
            // Option 1: Increase k
            let next_k_min_cost = calculate_min_cost(as_left, &bs_left[1..], memo);
            
            // Option 2: Take as many a's as possible (no point in leaving any)
            let next_b = bs_left[0];
            let mut next_a = as_left[0];
            let mut cum_sum = 0;
            let mut a_prefix_to_remove_count = 0;
            
            while cum_sum + next_a <= next_b {
                cum_sum += next_a;
                a_prefix_to_remove_count += 1;
                if a_prefix_to_remove_count >= as_left_count {
                    // No more a's left
                    break;
                }
                next_a = as_left[a_prefix_to_remove_count];
            }
            
            if a_prefix_to_remove_count == 0 {
                next_k_min_cost
            } else {
                let min_cost_of_remaining_as =
                    calculate_min_cost(&as_left[a_prefix_to_remove_count..], bs_left, memo);
                
                if let Some(rem_cost) = min_cost_of_remaining_as {
                    let this_move_cost = rem_cost + bs_left_count - 1;
                    
                    if let Some(next_k_cost) = next_k_min_cost {
                        Some(this_move_cost.min(next_k_cost))
                    } else {
                        Some(this_move_cost)
                    }
                } else {
                    next_k_min_cost
                }
            }
        };
        
        memo[bs_left_count][as_left_count] = Some(min_cost);
        min_cost
    }
}
