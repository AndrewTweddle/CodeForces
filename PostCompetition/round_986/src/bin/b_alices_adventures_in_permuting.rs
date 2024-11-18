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
        let params_str = line_iter.next().unwrap().unwrap();
        let params: Vec<i64> = params_str
            .split(' ')
            .map(|num_str| num_str.parse::<i64>().unwrap())
            .collect();
        let n = params[0];
        let b = params[1];
        let c = params[2];
        let answer = match (n, b, c) {
            // -------------------
            // Cases where n == 1:
            (1, _, 0) => 0, // 0
            (1, _, _) => 1, // b -> 0 (with b != 0)

            // -----------------------------
            // Cases where n > 1 and c >= n:
            //
            // Index: 0   1   ... (n-1)         n ... c
            // Array: c (c+b) ... (c + (n-1).b) | NB: last index is n - 1.
            //     -> 0   1   ... (n-1)         after n steps
            (n, _, c) if n <= c => n,

            // ------------------------------------
            // Cases where n > 1, c < n and b == 0:
            //
            // Index: 0 1 ... c (c+1) ... (n-1)
            // Array: c c ... c   c   ...   c
            //     -> 0 1 ... c   c   ...   c   after c steps; is a perm if n - 1 == c
            //     -> 0 1 ... c (c+1) c ... c   after c + 1 steps; is a perm if n - 1 == c + 1
            //     -> 0 1 ... c (c+2) c ... c   which will repeat, so no perm is possible
            (n, 0, c) if n == c + 1 => c,
            (n, 0, c) if n == c + 2 => c + 1,
            (_, 0, _) => -1,

            // -----------------------------
            // Cases where n > 1, c < n and b > 0:
            //
            // Suppose c + (k-1).b < n and c + kb >= n:
            // Hence k = 1 + floor((n - c - 1) / b)
            //
            // Index: 0    1    ...   ...       ...  n  ...
            // Array: c (c + b) ... (c+(k-1).b) ... ... c+kb  ...
            //     -> c (c + b) ... (c+(k-1).b) ... 0 1 ... (c-1) (c+1) ... (c+b-1) (c+b+1) ...
            //
            // k numbers are re-used, so # steps = n - k = n - floor((n - c - 1) / b) - 1
            _ => n - 1 - (n - c - 1) / b,
        };
        println!("{answer}");
    }
}
