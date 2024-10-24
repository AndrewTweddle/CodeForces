use std::io::BufRead;

type Memo = Vec<Option<usize>>;

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
        let topics: Vec<u32> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect();

        // Don't swap the first and the last
        let mut memo: Memo = vec![None; n + 1];
        let min_disturbance = calculate_inner_disturbance(&topics, &mut memo);

        println!("{min_disturbance}");
    }
}

fn calculate_min_disturbance(left: u32, right: u32, remaining: &[u32], memo: &mut Memo) -> usize {
    if remaining.is_empty() {
        return if left == right { 1 } else { 0 };
    }
    let num_rem = remaining.len();
    let next_left = remaining[0];
    let next_right = remaining[num_rem - 1];
    let unswapped_disturbance =
        calculate_disturbance(left, right, remaining, num_rem, next_left, next_right, memo);

    // Don't swap if the numbers are identical
    if next_left == next_right {
        return unswapped_disturbance;
    }

    let swapped_disturbance =
        calculate_disturbance(left, right, remaining, num_rem, next_right, next_left, memo);
    swapped_disturbance.min(unswapped_disturbance)
}

fn calculate_disturbance(
    left: u32,
    right: u32,
    remaining: &[u32],
    num_rem: usize,
    next_left: u32,
    next_right: u32,
    memo: &mut Memo,
) -> usize {
    let mut disturbance = 0;

    // Calculate what happens if we don't swap the next pair of indices
    if next_left == left {
        disturbance += 1;
    }
    if next_right == right {
        disturbance += 1;
    }
    if num_rem > 2 {
        disturbance += calculate_inner_disturbance(remaining, memo);
    } else if num_rem == 2 && next_left == next_right {
        disturbance += 1;
    }
    disturbance
}

fn calculate_inner_disturbance(remaining: &[u32], memo: &mut Memo) -> usize {
    let num_rem = remaining.len();
    if let Some(disturbance) = memo[num_rem] {
        disturbance
    } else {
        let disturbance = if num_rem < 2 {
            0
        } else {
            calculate_min_disturbance(
                remaining[0],
                remaining[num_rem - 1],
                &remaining[1..num_rem - 1],
                memo,
            )
        };
        memo[num_rem] = Some(disturbance);
        disturbance
    }
}
