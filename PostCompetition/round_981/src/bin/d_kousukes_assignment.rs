use std::collections::BTreeMap;
use std::io::BufRead;
use std::iter;

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
        let _n: usize = line_iter.next().unwrap().unwrap().parse::<usize>().unwrap();
        let cum_sums: Vec<i32> = iter::once(0)
            .chain(
                line_iter
                    .next()
                    .unwrap()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|num_str| num_str.parse::<i32>().unwrap()),
            )
            .scan(0, |cum_sum, element| {
                *cum_sum += element;
                Some(*cum_sum)
            })
            .collect();

        let mut segments: Vec<(usize, usize)> = Vec::new();
        let mut cum_sum_map = BTreeMap::<i32, Vec<usize>>::new();

        for (idx, &cum_sum) in cum_sums.iter().enumerate() {
            let indices = cum_sum_map.entry(cum_sum).or_default();
            if let Some(range) = indices.last().map(|&last_index| (last_index, idx)) {
                segments.push(range);
            }
            indices.push(idx);
        }

        // Ensure that the segments are sorted by decreasing end index (right to left by end index).
        segments.reverse();

        // As we work through this reversed vector, we will be counting non-overlapping
        // beautiful segments from right to left. Note that start indices may be out of order.

        // We don't want to compute the same suffix of the vector of segments more than once, so
        // memo-ize the maximum number of beautiful segments, keyed by the # of remaining segments.
        let seg_count = segments.len() + 1;
        let mut memo: Vec<Option<usize>> = vec![None; seg_count + 1];
        if seg_count > 0 {
            memo[0] = Some(0);
        }
        if seg_count > 1 {
            memo[1] = Some(1);
        }

        let max_beautiful_segments = calculate_max_non_overlapping(&segments, &mut memo);
        println!("{max_beautiful_segments}");
    }
}

fn calculate_max_non_overlapping(
    segments: &[(usize, usize)],
    memo: &mut Vec<Option<usize>>,
) -> usize {
    let seg_count = segments.len();
    if let Some(max_non_overlapping) = memo[seg_count] {
        return max_non_overlapping;
    }

    // NB: We can now assume that seg_count >= 2, since memo[0] and memo[1] are already seeded

    let curr = segments[0];
    let prev = segments[1];

    let max_ignoring_first = calculate_max_non_overlapping(&segments[1..], memo);

    let max_non_overlapping = if prev.1 <= curr.0 {
        // The first two segments don't overlap, so always include the first
        1 + max_ignoring_first
    } else {
        // There is an overlap, so consider both cases - where curr is either excluded or included

        // Skip over segments that still overlap with the current segment
        let index_of_next_not_overlapping = segments
            .iter()
            .enumerate()
            .skip(2)
            .find_map(|(index, &seg)| if seg.1 <= curr.0 { Some(index) } else { None });

        let max_if_included = if let Some(sub_range_start_index) = index_of_next_not_overlapping {
            1 + calculate_max_non_overlapping(&segments[sub_range_start_index..], memo)
        } else {
            // No other segments could be found
            1
        };
        max_if_included.max(max_ignoring_first)
    };
    memo[seg_count] = Some(max_non_overlapping);
    max_non_overlapping
}
