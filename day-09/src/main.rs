use std::collections::HashSet;

fn main() {
    // let input = include_str!("sample_input.txt");
    // let input = include_str!("sample_input2.txt");
    let input = include_str!("input.txt");
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut t_positions: HashSet<(i32, i32)> = HashSet::new();
    t_positions.insert(t);
    let input = input
        .lines()
        .map(|line| line.split_once(' ').expect("<dir> <num>"))
        .map(|(dir, num_str)| (dir, num_str.parse::<i32>().expect("integer")));
    for (dir, num) in input.clone() {
        // println!("## {} {} -- H:{:?}, T:{:?}", dir, num, h, t);
        for _ in 0..num {
            // move head
            // print!("H:{:?}", h);
            h = new_head_loc(h, dir);
            // print!(" +#{}->H:{:?}", n, h);

            // catch up tail
            t = new_tail_loc(h, t);
            // println!("    -> {:?}", t);

            t_positions.insert(t);
        }
    }
    println!("total tail positions = {}", t_positions.len());

    println!("**************************************************");
    let mut h = (0, 0);
    let mut knots = vec![(0, 0); 9];
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut part2_tail_positions: HashSet<(i32, i32)> = HashSet::new();

    for (dir, num) in input {
        // println!("## {} {} ##", dir, num);
        // println!("before:");
        // print_matrix(&to_matrix(
        //     &mut std::iter::once(&h).chain(knots.iter()),
        //     min_x,
        //     max_x,
        //     min_y,
        //     max_y,
        // ));
        // println!();
        for _ in 0..num {
            h = new_head_loc(h, dir);

            min_x = min_x.min(h.0);
            max_x = max_x.max(h.0);
            min_y = min_y.min(h.1);
            max_y = max_y.max(h.1);

            let mut prev_knot = h;
            for k in knots.iter_mut() {
                let new_pos = new_tail_loc(prev_knot, *k);
                if *k == new_pos {
                    break;
                }
                *k = new_pos;
                prev_knot = new_pos;
            }
            part2_tail_positions.insert(knots[8]);
        }
        // println!("after:");
        // print_matrix(&to_matrix(
        // print_matrix(&to_matrix(
        //     &mut std::iter::once(&h).chain(knots.iter()),
        //     min_x,
        //     max_x,
        //     min_y,
        //     max_y,
        // ));
        // println!("----------");
    }
    println!("part2 tail positions = {}", part2_tail_positions.len());
    // let mut sorted_positions = t_positions.iter().cloned().collect::<Vec<(i32, i32)>>();
    // sorted_positions.sort_unstable();
    // println!("{:?}", sorted_positions);
}

#[allow(dead_code)]
fn to_matrix(
    knots: &mut dyn Iterator<Item = &(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> Vec<Vec<char>> {
    const EMPTY: char = '·';
    let width = (max_x.abs_diff(min_x) + 1) as usize;
    let height = (max_y.abs_diff(min_y) + 1) as usize;
    let mut matrix = Vec::new();
    for _ in 0..=height {
        matrix.push(vec![EMPTY; width])
    }
    for (i, k) in knots.enumerate() {
        let r = (k.1 + min_y.abs()) as usize;
        let c = (k.0 + min_x.abs()) as usize;
        if matrix[r][c] == EMPTY {
            let ch = match i {
                0 => 'H',
                _ => i.to_string().chars().take(1).next().unwrap(),
            };
            matrix[r][c] = ch;
        }
    }
    matrix
}

#[allow(dead_code)]
fn print_matrix(matrix: &[Vec<char>]) {
    let mut m = matrix.to_owned();
    m.reverse();
    let rows = m
        .iter()
        .map(|row| row.iter().map(|cell| cell.to_string()))
        .map(String::from_iter);
    for row in rows {
        println!("{}", row);
    }
}

/// Gets the new location of head/a knot after moving it once in the specified direction.
fn new_head_loc(h: (i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "R" => (h.0 + 1, h.1),
        "L" => (h.0 - 1, h.1),
        "U" => (h.0, h.1 + 1),
        "D" => (h.0, h.1 - 1),
        _ => unreachable!("unexpected direction in input"),
    }
}

/// Gets the new location for `t` based on the current location of `h`.
fn new_tail_loc(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    let delta = ((h.0 - t.0), (h.1 - t.1));
    // + -> head is to the right, or above
    // println!("    T:{:?}, Delta:{:?}", t, delta);
    match delta {
        (-2, -2) => (t.0 - 1, t.1 - 1),
        (-2, -1) => (t.0 - 1, t.1 - 1),
        (-2, 0) => (t.0 - 1, t.1),
        (-2, 1) => (t.0 - 1, t.1 + 1),
        (-2, 2) => (t.0 - 1, t.1 + 1),
        (-1, -2) => (t.0 - 1, t.1 - 1),
        (-1, -1) => t,
        (-1, 0) => t,
        (-1, 1) => t,
        (-1, 2) => (t.0 - 1, t.1 + 1),
        (0, -2) => (t.0, t.1 - 1),
        (0, -1) => t,
        (0, 0) => t,
        (0, 1) => t,
        (0, 2) => (t.0, t.1 + 1),
        (1, -2) => (t.0 + 1, t.1 - 1),
        (1, -1) => t,
        (1, 0) => t,
        (1, 1) => t,
        (1, 2) => (t.0 + 1, t.1 + 1),
        (2, -2) => (t.0 + 1, t.1 - 1),
        (2, -1) => (t.0 + 1, t.1 - 1),
        (2, 0) => (t.0 + 1, t.1),
        (2, 1) => (t.0 + 1, t.1 + 1),
        (2, 2) => (t.0 + 1, t.1 + 1),
        _ => unreachable!("unexpected delta: {:?}", delta),
    }
}
