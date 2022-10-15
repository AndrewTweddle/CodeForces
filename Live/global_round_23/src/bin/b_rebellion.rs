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
        let bit_line = line_iter
            .next()
            .unwrap()
            .unwrap();
        let bits: Vec<bool> = bit_line
            .split(' ')
            .map(|digit| digit == "1")
            .collect();
        let mut j = n - 1;
        let mut op_count: usize = 0;
        for i in 0..n {
            if bits[i] {
                while j > i {
                    if !bits[j] {
                        op_count += 1;
                        j -= 1;
                        break;
                    }
                    j -= 1;
                }
                if j == i {
                    break;
                }
            }
        }

        println!("{}", op_count);
    }
}
