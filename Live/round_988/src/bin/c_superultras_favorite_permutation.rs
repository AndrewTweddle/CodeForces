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
        if n < 5 {
            println!("-1");
        } else if n == 5 {
            println!("2 4 5 3 1");
        } else if n == 6 {
            println!("6 2 4 5 3 1");
        } else {
            // Do all even numbers above 4. Then 2 4 5 3 1. Then all odd numbers from 7 on.
            for i in (6..=n).step_by(2) {
                print!("{i} ");
            }
            print!("2 4 5 3 1 ");
            let (penultimate, last) = if n % 2 == 0 {
                (n - 3, n - 1)
            } else {
                (n - 2, n)
            };
            for i in (7..=penultimate).step_by(2) {
                print!("{i} ");
            }
            println!("{last}");
        }
    }
}
