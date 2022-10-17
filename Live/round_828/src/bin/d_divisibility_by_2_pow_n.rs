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
        let n = line_iter.next().unwrap().unwrap().parse::<usize>().unwrap();
        let nums_str = line_iter.next().unwrap().unwrap();
        let nums_iter = nums_str
            .split(' ')
            .map(|num_str| num_str.parse::<u64>().unwrap());

        let mut init_pows_2: usize = 0;
        for num in nums_iter {
            let mut quotient = num;
            while quotient % 2 == 0 {
                init_pows_2 += 1;
                quotient /= 2;
            }
        }

        if init_pows_2 >= n {
            println!("0");
            continue;
        }

        let mut rem_pows_2: usize = n - init_pows_2;
        let mut ops_required = 0;

        let mut counts_by_pow: Vec<usize> = Vec::with_capacity(40);

        let mut quotient = n;
        while quotient > 0 {
            counts_by_pow.push(quotient);
            quotient /= 2;
        }

        let mut already_used_pows = 0;
        let pow_count = counts_by_pow.len() - 1;
        for pow_of_2 in (1..=pow_count).rev() {
            if rem_pows_2 == 0 {
                break;
            }
            // The number of values <= n divisible by 2^j includes those divisible by 2^(j+1),
            // 2^(j+2), etc. So the latter must be removed from the count,
            // since they've already been accounted for in earlier iterations
            let count_with_pow_of_2 = counts_by_pow[pow_of_2] - already_used_pows;
            already_used_pows += count_with_pow_of_2;
            for _ in 0..count_with_pow_of_2 {
                ops_required += 1;
                if rem_pows_2 <= pow_of_2 {
                    rem_pows_2 = 0;
                    break;
                } else {
                    rem_pows_2 -= pow_of_2;
                }
            }
        }

        if rem_pows_2 > 0 {
            println!("-1");
        } else {
            println!("{}", ops_required);
        }
    }
}
