use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};

fn main() {
    let stdout = stdout();
    let mut bw = BufWriter::new(stdout);

    let stdin = stdin();
    let mut br = BufReader::new(stdin);
    let mut num_tests_str = String::new();
    br.read_line(&mut num_tests_str).unwrap();
    let num_tests: usize = num_tests_str.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_tests {
        let mut test_case = String::new();
        br.read_line(&mut test_case).unwrap();
        let n: usize = test_case.trim().parse().unwrap();

        br.read_line(&mut test_case).unwrap();
        let a_s: Vec<usize> = test_case
            .trim()
            .split(' ')
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();

        let mut a_counts: Vec<usize> = vec![0_usize; n+1];
        for a in a_s {
            a_counts[a] += 1;
        }

        // Once there's one failure, all the rest are failures
        let mut has_failed: bool = false;
        for i in 0..=n {
            let mut num_moves: isize = -1;
            if !has_failed {
                let mut adj_a_counts = a_counts.clone();

                // Move all at value i up by 1...
                num_moves = adj_a_counts[i] as isize;

                for j_index in 1..=i {
                    // Ensure slot i-j is filled, else fill it from lower positions
                    let j = i - j_index;
                    if adj_a_counts[j] == 0 {
                        let mut is_j_filled: bool = false;
                        for k_index in 1..=j {
                            let k = j - k_index;
                            if adj_a_counts[k] != 0 {
                                adj_a_counts[k] -= 1;
                                num_moves += k_index as isize;
                                is_j_filled = true;
                                break;
                            }
                        }

                        if !is_j_filled {
                            num_moves = -1;
                            has_failed = true;
                            break;
                        }
                    }
                }
            }

            if i == 0 {
                write!(bw, "{}", num_moves).expect("Could not write to stdout");
            } else {
                write!(bw, " {}", num_moves).expect("Could not write to stdout");
            }
        }
        writeln!(bw, "").expect("Could not write to stdout");
    }
}
