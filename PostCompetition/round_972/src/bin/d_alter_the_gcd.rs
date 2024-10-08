use std::cmp::Ordering;
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

        // Prepend and append zeroes to both sets of numbers, to eliminate the edge case
        // of empty sets on the left and right, and to make the numbers one-based...
        let a: Vec<u32> = std::iter::once(0)
            .chain(
                line_iter
                    .next()
                    .unwrap()
                    .unwrap()
                    .split_whitespace()
                    .map(|i| i.parse().unwrap()),
            )
            .chain(std::iter::once(0))
            .collect();
        
        let b: Vec<u32> = std::iter::once(0)
            .chain(
                line_iter
                    .next()
                    .unwrap()
                    .unwrap()
                    .split_whitespace()
                    .map(|i| i.parse().unwrap()),
            )
            .chain(std::iter::once(0))
            .collect();
        
        let (solution, solution_count) = solve(n, a, b);

        println!("{solution} {solution_count}");
    }
}

fn solve(n: usize, a: Vec<u32>, b: Vec<u32>) -> (u32, usize) {
    let a_left_gcds: Vec<u32> = a
        .iter()
        .scan(*a.first().unwrap(), |acc, next_a| {
            *acc = gcd(*acc, *next_a);
            Some(*acc)
        })
        .collect();
    let a_right_gcds: Vec<u32> = a
        .iter()
        .rev()
        .scan(*a.first().unwrap(), |acc, next_a| {
            *acc = gcd(*acc, *next_a);
            Some(*acc)
        })
        .collect();
    let b_left_gcds: Vec<u32> = b
        .iter()
        .scan(*b.first().unwrap(), |acc, next_b| {
            *acc = gcd(*acc, *next_b);
            Some(*acc)
        })
        .collect();
    let b_right_gcds: Vec<u32> = b
        .iter()
        .rev()
        .scan(*b.first().unwrap(), |acc, next_b| {
            *acc = gcd(*acc, *next_b);
            Some(*acc)
        })
        .collect();
    
    let left_iter = a_left_gcds.iter().cloned().zip(b_left_gcds.iter().cloned()).enumerate();
    let right_iter = a_right_gcds.iter().cloned().zip(b_right_gcds.iter().cloned()).enumerate();
    
    let cache = Cache::new(&a, &b);
    
    let mut best_sum_of_gcds = 0;
    let mut best_count: usize = 0;
    
    for (l_index, (a_left_gcd, b_left_gcd)) in left_iter {
        // gcd's only get smaller as we consider more options, so exit early
        // whenever there is no chance of finding a better result.
        // The exception is when a gcd is zero (i.e. ignored), so also check for this.
        let l = l_index + 1;
        
        if a_left_gcd > 0 && b_left_gcd >0 && a_left_gcd + b_left_gcd < best_sum_of_gcds {
            continue;
        }
        for (r_index, (a_right_gcd, b_right_gcd)) in right_iter.clone() {
            let r = n - r_index;
            if r < l {
                break;
            }
            let mut a_gcd = gcd(a_left_gcd, a_right_gcd);
            if a_gcd > 0 && b_left_gcd > 0 && a_gcd + b_left_gcd < best_sum_of_gcds {
                continue;
            }
            let mut b_gcd = gcd(b_left_gcd, b_right_gcd);
            if a_gcd > 0 && b_gcd > 0 && a_gcd + b_gcd < best_sum_of_gcds {
                continue;
            }
            
            let (a_mid_gcd, b_mid_gcd) = cache.get_gcds_between_indices_inclusively(l, r);
            a_gcd = gcd(a_gcd, b_mid_gcd);
            if b_gcd > 0 && a_gcd + b_gcd < best_sum_of_gcds {
                continue;
            }
            b_gcd = gcd(b_gcd, a_mid_gcd);
            let sum_of_gcds = a_gcd + b_gcd;
            
            match sum_of_gcds.cmp(&best_sum_of_gcds) {
                Ordering::Less => {}
                Ordering::Equal => { best_count += 1; }
                Ordering::Greater => {
                    best_sum_of_gcds = sum_of_gcds;
                    best_count = 1;
                }
            }
        }
    }
    
    (best_sum_of_gcds, best_count)
}

struct Cache {
    pairs_by_level: Vec<Vec<(u32, u32)>>,
}

impl Cache {
    fn new(a_values: &[u32], b_values: &[u32]) -> Self {
        let mut count = a_values.len().max(b_values.len());
        let level_count = count.ilog2() as usize + 1;
        let mut pairs_by_level: Vec<Vec<(u32, u32)>> = Vec::with_capacity(level_count);
        pairs_by_level.push(a_values.iter().zip(b_values.iter()).map(|(&a, &b)| (a, b)).collect());
        count /= 2;
        while count != 0 {
            let prev_values = pairs_by_level.last().unwrap();
            pairs_by_level.push(
                prev_values
                    .chunks(2)
                    .filter_map(|chunk| {
                        if chunk.len() == 2 {
                            Some(
                                (
                                    gcd(chunk[0].0, chunk[1].0),
                                    gcd(chunk[0].1, chunk[1].1),
                                )
                            )
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
            count /= 2;
        }

        Self { pairs_by_level }
    }

    fn get_gcds_between_indices_inclusively(&self, left: usize, right: usize) -> (u32, u32) {
        self.get_gcds_between_indices_at_level(0, left, right)
    }

    fn get_gcds_between_indices_at_level(
        &self,
        level_index: usize,
        left: usize,
        right: usize,
    ) -> (u32, u32) {
        let level = &self.pairs_by_level[level_index];
        if left > right {
            panic!("The left index should never be greater than the right index");
        }
        if left >= level.len() {
            return (0, 0);
        }
        if left == right {
            return level[left];
        }
        if left % 2 == 1 {
            // Take this gcd on its own, then combine all remaining gcd's recursively
            let left_gcd = level[left];
            // Once a gcd is 1, it can't get any lower, so skip further calculations
            if left_gcd == (1, 1) {
                return left_gcd;
            }
            let rem_gcd = self.get_gcds_between_indices_at_level(level_index, left + 1, right);
            return (gcd(left_gcd.0, rem_gcd.0), gcd(left_gcd.1, rem_gcd.1));
        }
        if right % 2 == 0 {
            // Take this gcd on its own, then combine all remaining gcd's recursively
            let right_gcd = level[right];
            // Once a gcd is 1, it can't get any lower, so skip further calculations
            if right_gcd == (1, 1) {
                return right_gcd;
            }
            let rem_gcd = self.get_gcds_between_indices_at_level(level_index, left, right - 1);
            return (gcd(rem_gcd.0, right_gcd.0), gcd(rem_gcd.1, right_gcd.1));
        }
        // Get cumulative gcd's at the next level down
        self.get_gcds_between_indices_at_level(level_index + 1, left / 2, right / 2)
    }
}

fn gcd(m: u32, n: u32) -> u32 {
    let mut larger = m.max(n);
    let mut smaller = m.min(n);

    while smaller != 0 {
        larger %= smaller;
        std::mem::swap(&mut larger, &mut smaller);
    }
    larger
}

#[cfg(test)]
mod tests {
    mod gcd_tests {
        use crate::gcd;
        
        #[test]
        fn test_gcd_smaller_then_larger() {
            assert_eq!(gcd(6, 9), 3);
        }
        
        #[test]
        fn test_gcd_larger_then_smaller() {
            assert_eq!(gcd(6, 9), 3);
        }
        
        #[test]
        fn test_gcd_larger_then_zero() {
            assert_eq!(gcd(10, 0), 10);
        }
    }
    
    mod solver_tests {
        use crate::solve;
        
        #[test]
        fn test_case_1() {
            // Remember to prepend and append a zero
            let a = vec![0, 11, 4, 16, 17, 3, 24, 25, 8, 0];
            let b = vec![0, 8, 10, 4, 21, 17, 18, 25, 21, 0];
            let (gcd_total, count) = solve(8, a, b);
            assert_eq!(gcd_total, 2);
            assert_eq!(count, 36);
        }
        
        #[test]
        fn test_case_2() {
            // Remember to prepend and append a zero
            let a = vec![0, 6, 4, 24, 13, 0];
            let b = vec![0, 15, 3, 1, 14, 0];
            let (gcd_total, count) = solve(4, a, b);
            assert_eq!(gcd_total, 3);
            assert_eq!(count, 2);
        }
    }
}
