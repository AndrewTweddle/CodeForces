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
        let n_q_str = line_iter.next().unwrap().unwrap();
        let (n_str, q_str) = n_q_str.split_once(' ').unwrap();
        let (_n, q) = (n_str.parse::<usize>().unwrap(), q_str.parse::<usize>().unwrap());
        let nums_str = line_iter.next().unwrap().unwrap();
        let nums_iter = nums_str
            .split(' ')
            .map(|num_str| num_str.parse::<u64>().unwrap());

        let mut total:u64 = 0;
        let mut odd_count: u64 = 0;
        let mut even_count: u64 = 0;

        for num in nums_iter {
            total += num;
            if num % 2 == 0 {
                even_count += 1;
            } else {
                odd_count += 1;
            }
        }

        let query_iter = (0..q).map(|_| {
            let query_line_str = line_iter.next().unwrap().unwrap();
            let (type_j_str, x_j_str) = query_line_str.split_once(' ').unwrap();
            (type_j_str.parse::<u64>().unwrap(), x_j_str.parse::<u64>().unwrap())
        });

        for query in query_iter {
            if query.0 == 0 {
                total += query.1 * even_count;
                // Adding odd to all the even numbers will make them odd...
                if query.1 % 2 == 1 {
                    odd_count += even_count;
                    even_count = 0;
                }
            } else {
                total += query.1 * odd_count;
                // Adding an odd number to all the odd numbers will make them even...
                if query.1 % 2 == 1 {
                    even_count += odd_count;
                    odd_count = 0;
                }
            }

            println!("{}", total);
        }
    }
}
