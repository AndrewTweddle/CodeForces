use std::collections::HashMap;
use std::io::{BufRead, Lines, StdinLock};
use std::iter;

type Hurdle = (u32, u32);

struct Section {
    hurdle_length: u32,
    cum_prior_power_ups: Vec<u32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    section_index: usize,
    k: u32,
}

type Memo = HashMap<State, Option<u32>>;
// The memo value is the minimum number of power-ups needed to reach the end position (L)
// from the section prior to a hurdle, given a jump power of k.
// This is before any power-ups have been taken in that section, so it's over all possible
// ways of taking power-ups in the current section and all remaining sections.

fn main() {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let mut line_iter = stdin_lock.lines();
    let t = line_iter
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
    for _ in 0..t {
        let n_m_l_str = line_iter.next().unwrap().unwrap();
        let n_m_l_vec: Vec<usize> = n_m_l_str
            .split(' ')
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();
        let n = n_m_l_vec[0];
        let m = n_m_l_vec[1];
        let _l = n_m_l_vec[2];

        let hurdles = parse_hurdles(&mut line_iter, n);
        let power_ups =
            parse_power_ups_by_hurdle_as_cumulative_decreasing_values(&mut line_iter, m, &hurdles);
        let sections: Vec<Section> = hurdles
            .into_iter()
            .zip(power_ups.into_iter())
            .map(|((hurdle_l, hurdle_r), cum_prior_power_ups)| Section {
                hurdle_length: hurdle_r - hurdle_l + 1,
                cum_prior_power_ups,
            })
            .collect();

        let mut memo: Memo = Memo::new();
        let maybe_min_power_ups = solve(0, 1, &sections, &mut memo);
        if let Some(min_power_ups) = maybe_min_power_ups {
            println!("{min_power_ups}")
        } else {
            println!("-1");
        }
    }
}

fn solve(section_index: usize, k: u32, sections: &[Section], memo: &mut Memo) -> Option<u32> {
    let state = State { section_index, k };
    if let Some(state) = memo.get(&state) {
        return *state;
    }

    // There is no need to take any power-ups when in the section after the final hurdle:
    if section_index == sections.len() {
        return Some(0);
    }

    let next_section = &sections[section_index];
    let mut min_power_ups: Option<u32> = None;

    // Now try taking power-ups, first taking none, then taking from most to least valuable:
    let power_ups_iter = iter::once(0)
        .chain(next_section.cum_prior_power_ups.iter().cloned())
        .enumerate();

    for (i, cum_value_taken) in power_ups_iter {
        let num_taken = i as u32;
        let new_k = k + cum_value_taken;
        if new_k > next_section.hurdle_length {
            let power_ups_taken = solve(section_index + 1, new_k, sections, memo);
            match (power_ups_taken, min_power_ups) {
                (Some(extra_taken), None) => min_power_ups = Some(num_taken + extra_taken),
                (Some(extra_taken), Some(min_taken)) if min_taken > num_taken + extra_taken => {
                    min_power_ups = Some(num_taken + extra_taken)
                }
                _ => {}
            }
        }
    }

    memo.insert(state, min_power_ups);
    min_power_ups
}

fn parse_hurdles(line_iter: &mut Lines<StdinLock>, n: usize) -> Vec<(u32, u32)> {
    let mut hurdles: Vec<Hurdle> = Vec::with_capacity(n);
    for _ in 0..n {
        let l_r_str = line_iter.next().unwrap().unwrap();
        let (l_str, r_str) = l_r_str.split_once(' ').unwrap();
        let lr_pair = (l_str.parse::<u32>().unwrap(), r_str.parse::<u32>().unwrap());
        hurdles.push(lr_pair);
    }
    hurdles
}

fn parse_power_ups_by_hurdle_as_cumulative_decreasing_values(
    line_iter: &mut Lines<StdinLock>,
    m: usize,
    hurdles: &[Hurdle],
) -> Vec<Vec<u32>> {
    let mut power_ups: Vec<Vec<u32>> = vec![Vec::default(); hurdles.len()];
    if hurdles.is_empty() {
        return power_ups;
    }
    let mut next_hurdle_index = 0;
    let mut process_more_hurdles = !hurdles.is_empty();

    for _ in 0..m {
        let x_v_str = line_iter.next().unwrap().unwrap();

        // Don't waste time parsing the line if we are done processing hurdles
        if !process_more_hurdles {
            continue;
        }

        let (x_str, v_str) = x_v_str.split_once(' ').unwrap();
        let (x, v) = (x_str.parse::<u32>().unwrap(), v_str.parse::<u32>().unwrap());

        // Find the next hurdle (possibly the same hurdle again), that the power-up precedes:
        let mut next_hurdle = &hurdles[next_hurdle_index];
        while next_hurdle.1 < x {
            next_hurdle_index += 1;
            if next_hurdle_index == hurdles.len() {
                // If there are no more hurdles to get over, then no more power-ups are needed
                process_more_hurdles = false;
                break;
            }
            next_hurdle = &hurdles[next_hurdle_index];
        }

        if process_more_hurdles {
            power_ups[next_hurdle_index].push(v);
        }
    }

    // All power-ups before a particular hurdle will be arranged in decreasing order of value.
    // Then the power-up values are replaced by the cumulative power-up values.
    // This way the most valuable power-ups are used first, and no more than necessary.
    for vals in &mut power_ups {
        vals.sort();
        vals.reverse();
        let mut cum = 0;
        for v in vals.iter_mut() {
            cum += *v;
            *v = cum;
        }
    }
    power_ups
}
