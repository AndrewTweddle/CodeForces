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

        // Convert permutations to be zero-based instead of 1-based, to use as array indices
        let permutation: Vec<usize> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<usize>().unwrap() - 1)
            .collect();
        
        // A permutation can be broken into distinct sub-permutations (cycles).
        // It always takes longer to join cycles together first, then break them into
        // simple (length 1 or 2) sub-cycles. So each cycle can be calculated independently.
        
        let mut total_operations: u32 = 0;
        let mut is_visited = vec![false; n];
        
        permutation.iter().enumerate().for_each(|(i, &j)| {
            if !is_visited[i] {
                // Determine the length of the cycle (sub-permutation) that includes element i.
                // Mark elements of the cycle as having been visited, to avoid counting them again.
                is_visited[i] = true;
                let mut cycle_length: u32 = 1;
                let mut next = j;
                while next != i {
                    cycle_length += 1;
                    is_visited[next] = true;
                    next = permutation[next];
                }
                total_operations += calculate_num_operations(cycle_length);
            }
        });
        println!("{total_operations}");
    }
}

fn calculate_num_operations(cycle_length: u32) -> u32 {
    // The following calculation has outputs of 0, 0, 1, 1, 2, 2, ... for inputs of 1, 2, 3, 4, ...
    // This corresponds to the minimum number of swap operations needed to break that cycle down
    // into cycles of length 1 or 2 (i.e. simple permutations).
    (cycle_length + 1) / 2 - 1
}
