use std::io::BufRead;

const LETTERS: usize = 5;
const NAME: [char; LETTERS] = ['n', 'a', 'r', 'e', 'k'];
type Memo = Vec<[Option<isize>; LETTERS]>;

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
        let n_m_line = line_iter.next().unwrap().unwrap();
        let mut n_m_iter = n_m_line.split_whitespace();
        let n: usize = n_m_iter.next().unwrap().parse().unwrap();
        // Don't need the following:
        // let m: usize = n_m_iter.next().unwrap().parse().unwrap();
        let strs: Vec<String> = (0..n).map(|_| line_iter.next().unwrap().unwrap()).collect();
        let mut scores_by_pos_memo: Memo = vec![[None; LETTERS]; strs.len()];
        let score = branch(&strs, 0, &mut scores_by_pos_memo);
        println!("{score}");
    }
}

fn branch(strs: &[String], pos: usize, scores_memo: &mut Memo) -> isize {
    if strs.is_empty() {
        // If the name was not completed, then the letters in the partial name count negative
        return -(pos as isize);
    }
    let memo_index = scores_memo.len() - strs.len();
    if let Some(memoized_score) = scores_memo[memo_index][pos] {
        return memoized_score;
    }
    // Try including this string...
    let mut next_pos = pos;
    let mut next_score: isize = 0;
    for ch in strs[0].chars() {
        if ch == NAME[next_pos] {
            next_pos += 1;
            if next_pos == NAME.len() {
                next_score += 5;
                next_pos = 0;
            }
        } else if NAME.contains(&ch) {
            next_score -= 1;
        }
    }
    
    // If we are in the same position in the name as before,
    // then only keep the string if it improves the score...
    let score = if next_pos == pos {
        let sub_score = branch(&strs[1..], pos, scores_memo);
        if next_score > 0 {
            // Definitely keep this string
            next_score + sub_score
        } else {
            // Definitely exclude this string
            sub_score
        }
    } else {
        // It's not certain whether to include this string or not. Try both branches...
        let score_inclusive = next_score + branch(&strs[1..], next_pos, scores_memo);
        let score_exclusive = branch(&strs[1..], pos, scores_memo);
        score_inclusive.max(score_exclusive)
    };
    scores_memo[memo_index][pos] = Some(score);
    score
}