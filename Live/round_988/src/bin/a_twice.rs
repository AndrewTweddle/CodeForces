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
        let _n = line_iter.next().unwrap().unwrap().parse::<usize>().unwrap();
        let nums_str = line_iter.next().unwrap().unwrap();
        let nums_iter = nums_str
            .split(' ')
            .map(|num_str| num_str.parse::<usize>().unwrap());
        let mut counts = [0_usize; 21];
        for i in nums_iter {
            counts[i] += 1;
        }
        let score: usize = counts.iter().map(|cnt| cnt / 2).sum();
        println!("{}", score);
    }
}
