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
        let n: usize = line_iter.next().unwrap().unwrap().parse().unwrap();
        let mut max_w: usize = 0;
        let mut max_h: usize = 0;
        for _ in 0..n {
            let line = line_iter.next().unwrap().unwrap();
            let (w_str, h_str) = line.split_once(' ').unwrap();
            let w = w_str.parse::<usize>().unwrap();
            let h = h_str.parse::<usize>().unwrap();
            if w > max_w {
                max_w = w;
            }
            if h > max_h {
                max_h = h;
            }
        }
        let perimeter = 2 * (max_h + max_w);
        println!("{perimeter}");
    }
}
