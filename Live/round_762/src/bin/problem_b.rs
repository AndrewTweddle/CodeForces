use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};

fn main() {
    let stdout = stdout();
    let mut bw = BufWriter::new(stdout);

    let stdin = stdin();
    let mut br = BufReader::new(stdin);
    let mut num_tests_str = String::new();
    br.read_line(&mut num_tests_str).unwrap();
    let num_tests: usize = num_tests_str.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_tests {
        let mut n_string = String::new();
        br.read_line(&mut n_string).unwrap();
        let n: usize = n_string.trim_end().parse().unwrap();
        let squares = count_powers_less_than(2, n);
        let cubes = count_powers_less_than(3, n);
        let sixth_powers = count_powers_less_than(6, n);
        let answer = squares + cubes - sixth_powers;
        writeln!(bw, "{}", answer).expect("Could not write to stdout");
    }
}

fn count_powers_less_than(exponent: u32, n: usize) -> usize {
    let count: usize = (1..=n)
        .into_iter()
        .take_while(|i| i.pow(exponent) <= n)
        .count();
    count
}