use std::io::Write;
use std::io::{stdin, stdout, BufRead};

fn main() {
    let stdin = stdin();
    let stdin_lock = stdin.lock();
    let line_iter = stdin_lock.lines().map(|ln| ln.unwrap());
    let mut lock = stdout().lock();
    solve(line_iter, &mut lock);
}

fn solve(mut line_iter: impl Iterator<Item = String>, outputs: &mut impl Write) {
    let t = line_iter.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let _n = line_iter.next().unwrap().parse::<usize>();
        let a: Vec<u64> = line_iter
            .next()
            .unwrap()
            .split(' ')
            .map(|i| i.parse::<u64>().unwrap())
            .collect();
        let mut a_sorted = a.clone();
        a_sorted.sort();
        let min = *a_sorted.first().unwrap();
        let max = *a_sorted.last().unwrap();
        if min == max {
            writeln!(outputs, "-1").unwrap();
        } else {
            let median = a_sorted[a_sorted.len() / 2];
            let (b, c): (Vec<u64>, Vec<u64>) = if median > min {
                a.iter().partition(|&i| *i < median)
            } else {
                a.iter().partition(|&i| *i <= median)
            };
            writeln!(outputs, "{} {}", b.len(), c.len()).unwrap();
            let b_strs: Vec<String> = b.iter().map(|i| i.to_string()).collect();
            let c_strs: Vec<String> = c.iter().map(|i| i.to_string()).collect();
            writeln!(outputs, "{}", b_strs.join(" ")).unwrap();
            writeln!(outputs, "{}", c_strs.join(" ")).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use std::io::BufWriter;

    #[test]
    fn test_solve() {
        let input_str = "5
3
2 2 2
5
1 2 3 4 5
3
1 3 5
7
1 7 7 2 9 1 4
5
4 8 12 12 4";

        let expected_output_str = "-1
2 3
1 2
3 4 5
1 2
1
3 5
3 4
1 2 1
7 7 9 4
2 3
4 4
8 12 12
";

        let input_iter = input_str.lines().map(|s| s.to_string());
        let mut output_writer = BufWriter::new(Vec::new());
        solve(input_iter, &mut output_writer);
        let actual_output = String::from_utf8(output_writer.into_inner().unwrap()).unwrap();
        assert_eq!(expected_output_str, actual_output);
    }
}
