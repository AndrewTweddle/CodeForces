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
        let mut test_case = String::new();
        br.read_line(&mut test_case).unwrap();
        let (a_str, s_str) = test_case.trim().split_once(' ').unwrap();
        let mut a: i64 = a_str.parse().unwrap();
        let mut s: i64 = s_str.parse().unwrap();
        let mut b: i64 = 0;
        let mut pow_10: i64 = 1;

        while a > 0 || s > 0 {
            let a_digit = a % 10;
            let mut s_digit = s % 10;
            let mut b_digit = s_digit - a_digit;
            if b_digit >= 0 {
                s /= 10;
            } else {
                s_digit = s % 100;
                b_digit = s_digit - a_digit;
                if b_digit < 0 || b_digit > 9 {
                    b = -1;
                    break;
                }
                s /= 100;
            }
            a /= 10;
            b += b_digit * pow_10;
            pow_10 *= 10;
        }

        writeln!(bw, "{}", b).expect("Could not write to stdout");
    }
}
