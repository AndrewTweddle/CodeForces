use std::io::{stdin, stdout, BufRead, Write};

fn main() {
    let stdin = stdin();
    let stdin_lock = stdin.lock();
    let line_iter = stdin_lock.lines().map(|ln| ln.unwrap());
    let mut lock = stdout().lock();
    solve(line_iter, &mut lock);
}

fn solve(mut line_iter: impl Iterator<Item = String>, writer: &mut impl Write) {
    let t = line_iter.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let _n = line_iter.next().unwrap().parse::<usize>();
        let a: Vec<u64> = line_iter
            .next()
            .unwrap()
            .split(' ')
            .map(|i| i.parse::<u64>().unwrap())
            .collect();
        if a.len() <= 1 {
            writeln!(writer, "-1").unwrap();
            continue;
        }

        // Look for the first 2 numbers that differ
        let a1 = a[0];
        if let Some(&a2) = a.iter().skip(1).find(|&ai| *ai != a1) {
            // All numbers <= the lower of the two will go in b, the rest will go in c.
            // Since all numbers in c are higher than all numbers in b, they divide none of them.
            let cutoff = a1.min(a2);
            let (b, c): (Vec<u64>, Vec<u64>) = a.iter().partition(|&i| *i <= cutoff);
            writeln!(writer, "{} {}", b.len(), c.len()).unwrap();
            let b_strs: Vec<String> = b.iter().map(|i| i.to_string()).collect();
            let c_strs: Vec<String> = c.iter().map(|i| i.to_string()).collect();
            writeln!(writer, "{}", b_strs.join(" ")).unwrap();
            writeln!(writer, "{}", c_strs.join(" ")).unwrap();
        } else {
            writeln!(writer, "-1").unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use std::io::BufWriter;

    fn solve_for_str(input_str: &str) -> String {
        let input_iter = input_str.lines().map(|s| s.to_string());
        let mut output_writer = BufWriter::new(Vec::new());
        solve(input_iter, &mut output_writer);
        String::from_utf8(output_writer.into_inner().unwrap()).unwrap()
    }

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
1 4
1
2 3 4 5
1 2
1
3 5
2 5
1 1
7 7 2 9 4
2 3
4 4
8 12 12
";

        let actual_output = solve_for_str(input_str);
        assert_eq!(expected_output_str, actual_output);
    }
}
