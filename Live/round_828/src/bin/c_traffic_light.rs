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
        let n_c_str = line_iter.next().unwrap().unwrap();
        let (n_str, c_str) = n_c_str.split_once(' ').unwrap();
        let (_n, c) = (n_str.parse::<usize>().unwrap(), c_str.chars().next().unwrap());
        let s = line_iter.next().unwrap().unwrap();

        // If already at green, we can cross immediately
        if c == 'g' {
            println!("0");
            continue;
        }

        let mut first_g_found = false;
        let mut dist_to_first_g: usize = 0;
        let mut dist_to_green: usize = 0;
        let mut searching_for_green = false;
        let mut max_dist_to_green: usize = 0;

        for symbol in s.chars() {
            if searching_for_green {
                dist_to_green += 1;
            }
            if !first_g_found {
                dist_to_first_g += 1;
            }
            if symbol == 'g' {
                first_g_found = true;
                if searching_for_green {
                    if dist_to_green > max_dist_to_green {
                        max_dist_to_green = dist_to_green;
                    }
                    searching_for_green = false;
                }
            }
            else if !searching_for_green && symbol == c {
                searching_for_green = true;
                dist_to_green = 0;
            }
        }

        // Handle wrap-around from end to the beginning
        if searching_for_green {
            dist_to_green += dist_to_first_g;
            if dist_to_green > max_dist_to_green {
                max_dist_to_green = dist_to_green;
            }
        }
        println!("{0}", max_dist_to_green);
    }
}
