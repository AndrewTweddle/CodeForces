use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let mut line_iter = stdin_lock.lines();
    let t = line_iter.next().unwrap().unwrap().as_str().parse::<usize>().unwrap();
    for _ in 0..t {
        // read n and c
        let line1 = line_iter
            .next()
            .unwrap()
            .unwrap();
        let (n_str, c_str) = line1
            .split_once(' ')
            .unwrap();
        let _n = n_str.parse::<usize>().unwrap();
        let c = c_str.parse::<usize>().unwrap();

        // read orbits
        let mut orbit_counts = [0_usize; 101];

        let line2 = line_iter
            .next()
            .unwrap()
            .unwrap();
        let orbit_iter = line2
            .split(' ')
            .map(|orbit_str| orbit_str.parse::<usize>().unwrap());
        for orbit in orbit_iter {
            orbit_counts[orbit] += 1;
        }

        let mut min_cost: usize = 0;
        for orbit_count in orbit_counts {
            if orbit_count > 0 {
                min_cost += orbit_count.min(c);
            }
        }

        println!("{}", min_cost);
    }
}