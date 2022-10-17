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
        let a_line = line_iter.next().unwrap().unwrap();
        let a_iter = a_line
            .split(' ')
            .map(|a_str| a_str.parse::<usize>().unwrap());

        // In operation i, choose the index of that a_i which has value n + 1 - i.
        // If i is in the set {1,...,n} then so is n + 1 - i, so there is such a unique a_i.
        // If we were only adding i to the value at this index,
        // then all the values would become n + 1 after the n operations.
        // Even this would be sufficient to eliminate all inversions.
        // By also adding i to the values at all subsequent indices as well
        // we don't change their inversion statuses relative to one another,
        // but we do make them even larger than numbers at earlier indices.

        let mut ops = vec![0_usize; n];

        for (index, a_i) in a_iter.enumerate() {
            let i = n - a_i; // ops is zero-based, so use n not n + 1
            ops[i] = index + 1; // index is zero-based, but we need it to be one-based
        }

        print!("{}", ops[0]);
        for index in ops[1..].iter() {
            print!(" {}", index);
        }
        println!();
    }
}
