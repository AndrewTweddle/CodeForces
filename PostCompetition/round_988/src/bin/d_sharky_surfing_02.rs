use std::collections::BTreeMap;
use std::io::{BufRead, Lines, StdinLock};

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
struct Hurdle {
    left: u32,
    min_k: u32,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
struct PowerUp {
    x: u32,
    v: u32,
}

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
        let power_ups = parse_power_ups(&mut line_iter, m);

        let maybe_min_power_ups = solve(hurdles, power_ups);
        if let Some(min_power_ups) = maybe_min_power_ups {
            println!("{min_power_ups}")
        } else {
            println!("-1");
        }
    }
}

fn solve(hurdles: Vec<Hurdle>, power_ups: Vec<PowerUp>) -> Option<u32> {
    let mut min_power_ups: u32 = 0;
    let mut k: u32 = 1;
    let mut power_up_count = BTreeMap::<u32, u32>::new();
    let mut power_up_iter = power_ups.iter().peekable();

    for h in hurdles {
        if k >= h.min_k {
            continue;
        }

        // Consider power-ups before the next hurdle, including all power-ups that could have
        // been taken earlier, but weren't needed (at the time) to get over previous hurdles
        while let Some(power_up) = power_up_iter.peek() {
            if power_up.x < h.left {
                power_up_count
                    .entry(power_up.v)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                power_up_iter.next();
            } else {
                break;
            }
        }

        while k < h.min_k {
            if let Some((v, count)) = power_up_count.pop_last() {
                let power_ups_needed = (h.min_k - k + v - 1) / v;
                let power_ups_taken = power_ups_needed.min(count);
                min_power_ups += power_ups_taken;
                k += power_ups_taken * v;
                if power_ups_taken != count {
                    // We didn't take all the power-ups, so put the rest back...
                    power_up_count.insert(v, count - power_ups_taken);
                }
            } else {
                return None;
            }
        }
    }
    Some(min_power_ups)
}

fn parse_hurdles(line_iter: &mut Lines<StdinLock>, n: usize) -> Vec<Hurdle> {
    let mut hurdles: Vec<Hurdle> = Vec::with_capacity(n);
    for _ in 0..n {
        let l_r_str = line_iter.next().unwrap().unwrap();
        let (l_str, r_str) = l_r_str.split_once(' ').unwrap();
        let (l, r) = (l_str.parse::<u32>().unwrap(), r_str.parse::<u32>().unwrap());
        let min_k = r - l + 2;
        hurdles.push(Hurdle { left: l, min_k });
    }
    hurdles
}

fn parse_power_ups(line_iter: &mut Lines<StdinLock>, m: usize) -> Vec<PowerUp> {
    let power_ups: Vec<PowerUp> = (0..m)
        .map(|_| {
            let x_v_str = line_iter.next().unwrap().unwrap();
            let (x_str, v_str) = x_v_str.split_once(' ').unwrap();
            let (x, v) = (x_str.parse::<u32>().unwrap(), v_str.parse::<u32>().unwrap());
            PowerUp { x, v }
        })
        .collect();
    power_ups
}
