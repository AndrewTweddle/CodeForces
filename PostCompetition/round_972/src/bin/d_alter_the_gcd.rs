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
    let a_cache = Cache::new(a);
    let b_cache = Cache::new(b);
    
    let mut best_sum_of_gcds = 0;
    let mut best_count: usize = 0;
    
    for l in 1..=n {
        let a_left_gcd = a_cache.get_gcds_between_indices_inclusively(0, l - 1);
        let b_left_gcd = b_cache.get_gcds_between_indices_inclusively(0, l - 1);
        for r in l..=n {
            let a_mid_gcd = a_cache.get_gcds_between_indices_inclusively(l, r);
            let a_right_gcd = a_cache.get_gcds_between_indices_inclusively(r + 1, n + 1);
            let b_mid_gcd = b_cache.get_gcds_between_indices_inclusively(l, r);
            let b_right_gcd = b_cache.get_gcds_between_indices_inclusively(r + 1, n + 1);
            let sum_of_gcds = gcd(a_left_gcd, gcd(b_mid_gcd, a_right_gcd))
                + gcd(b_left_gcd, gcd(a_mid_gcd, b_right_gcd));
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
    pairs_by_level: Vec<Vec<u32>>,
}

impl Cache {
    fn new(values: Vec<u32>) -> Self {
        let mut count = values.len();
        let level_count = count.ilog2() as usize + 1;
        let mut pairs_by_level = Vec::with_capacity(level_count);
        pairs_by_level.push(values);
        count /= 2;
        while count != 0 {
            let prev_values = pairs_by_level.last().unwrap();
            pairs_by_level.push(
                prev_values
                    .chunks(2)
                    .filter_map(|chunk| {
                        if chunk.len() == 2 {
                            Some(gcd(chunk[0], chunk[1]))
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

    fn get_gcds_between_indices_inclusively(&self, left: usize, right: usize) -> u32 {
        self.get_gcds_between_indices_at_level(0, left, right)
    }

    fn get_gcds_between_indices_at_level(
        &self,
        level_index: usize,
        left: usize,
        right: usize,
    ) -> u32 {
        let level = &self.pairs_by_level[level_index];
        if left > right {
            panic!("The left index should never be greater than the right index");
        }
        if left >= level.len() {
            return 0;
        }
        if left == right {
            return level[left];
        }
        if left % 2 == 1 {
            // Take this gcd on its own, then combine all remaining gcd's recursively
            let left_gcd = level[left];
            let rem_gcd = self.get_gcds_between_indices_at_level(level_index, left + 1, right);
            return gcd(left_gcd, rem_gcd);
        }
        if right % 2 == 0 {
            // Take this gcd on its own, then combine all remaining gcd's recursively
            let right_gcd = level[right];
            let rem_gcd = self.get_gcds_between_indices_at_level(level_index, left, right - 1);
            return gcd(rem_gcd, right_gcd);
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
