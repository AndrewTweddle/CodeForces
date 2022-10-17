use std::io::BufRead;
use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let stdout = std::io::stdout();
    let stdout_lock = stdout.lock();
    let mut bw = std::io::BufWriter::new(stdout_lock);

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
        let a_iter = a_line.split(' ').map(|a_str| a_str.parse::<usize>().unwrap());

        let mut i: usize = 0;
        let mut sum_of_i: usize = 0;
        let mut last_val: usize = 0;

        'outer: for (index, a) in a_iter.enumerate() {
            let mut new_val = a + sum_of_i;
            while new_val < last_val && i < n {
                if i == 0 {
                    write!(bw, "{}", index + 1).unwrap();
                } else {
                    write!(bw, " {}", index + 1).unwrap();
                }
                i += 1;
                sum_of_i += i;
                new_val += i;
                if i == n {
                    break 'outer;
                }
            }
            last_val = new_val;
        }

        if i == 0 {
            write!(bw, "{}", n).unwrap();
            i = 1;
        }

        for _ in i..n {
            write!(bw, " {}", n).unwrap();
        }
        writeln!(bw).unwrap();
    }
}
