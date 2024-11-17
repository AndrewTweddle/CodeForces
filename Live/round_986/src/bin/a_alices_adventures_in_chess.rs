use std::io::BufRead;
use std::iter;

type Instruction = (i32, i32);

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
        let params: Vec<i32> = params_str
            .split(' ')
            .map(|num_str| num_str.parse::<i32>().unwrap())
            .collect();
        let _n = params[0];
        let a = params[1];
        let b = params[2];

        let moves = line_iter.next().unwrap().unwrap();
        let instructions: Vec<Instruction> = moves
            .chars()
            .map(|ch| match ch {
                'N' => (0, 1),
                'E' => (1, 0),
                'S' => (0, -1),
                'W' => (-1, 0),
                _ => panic!("Invalid instruction"),
            })
            .collect();

        let cum_offsets: Vec<(i32, i32)> = iter::once((0, 0))
            .chain(
                instructions
                    .iter()
                    .scan((0_i32, 0_i32), |cum_offset, &instr| {
                        cum_offset.0 += instr.0;
                        cum_offset.1 += instr.1;
                        Some(*cum_offset)
                    }),
            )
            .collect();
        let &(offset_x, offset_y) = cum_offsets.last().unwrap();

        let found = cum_offsets.iter().any(|(cumx, cumy)| {
            // Find the target position to be at, to reach the red queen after this subset of
            // instructions, given k complete cycles of instructions already completed, where k >= 0
            let target_x = a - cumx;
            let target_y = b - cumy;

            if offset_x == 0 {
                if target_x != 0 {
                    false
                } else if offset_y == 0 {
                    target_y == 0
                } else {
                    target_y % offset_y == 0 && target_y / offset_y >= 0
                }
            } else {
                // Work out k
                if target_x % offset_x == 0 {
                    let k = target_x / offset_x;
                    if k < 0 {
                        false
                    } else {
                        offset_y * k == target_y
                    }
                } else {
                    false
                }
            }
        });
        if found {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
