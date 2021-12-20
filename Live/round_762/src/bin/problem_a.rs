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
        let mut string_to_test = String::new();
        br.read_line(&mut string_to_test).unwrap();
        let str_to_test = string_to_test.trim_end();
        let is_square: bool = if str_to_test.len() % 2 == 1 {
            false
        } else {
            let (str1, str2) = str_to_test.split_at(str_to_test.len() / 2);
            str1 == str2
        };
        writeln!(bw, "{}", if is_square { "YES" } else { "NO" })
            .expect("Could not write to stdout");
    }
}