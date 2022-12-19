use std::collections::HashSet;

fn main() {
    // let input = include_str!("sample_input.txt");
    let input = include_str!("input.txt");
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut t_positions: HashSet<(i32, i32)> = HashSet::new();
    t_positions.insert(t);
    for (dir, num) in input
        .lines()
        .map(|line| line.split_once(' ').expect("<dir> <num>"))
        .map(|(dir, num_str)| (dir, num_str.parse::<i32>().expect("integer")))
    {
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
    // let mut sorted_positions = t_positions.iter().cloned().collect::<Vec<(i32, i32)>>();
    // sorted_positions.sort_unstable();
    // println!("{:?}", sorted_positions);
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
        (-2, -1) => (t.0 - 1, t.1 - 1),
        (-2, 0) => (t.0 - 1, t.1),
        (-2, 1) => (t.0 - 1, t.1 + 1),
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
        (2, -1) => (t.0 + 1, t.1 - 1),
        (2, 0) => (t.0 + 1, t.1),
        (2, 1) => (t.0 + 1, t.1 + 1),
        _ => unreachable!("unexpected delta: {:?}", delta),
    }
}
