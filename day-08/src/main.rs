fn print_grid(grid: &Vec<Vec<i8>>) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}

fn main() {
    // let input = include_str!("sample_input.txt");
    let input = include_str!("input.txt");
    let grid: Vec<Vec<i8>> = input
        .lines()
        .map(|line| Vec::from_iter(line.chars().map(|ch| ch as i8 - '0' as i8)))
        .collect();
    print_grid(&grid);

    let mut visible_count = 0;
    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let height = grid[r][c];
            let is_border = (r == 0) || (c == 0) || (r == last_row) || (c == last_col);
            let this_row = &grid[r];
            let (left, right) = this_row.split_at(c);
            let left_max = left.iter().max().unwrap_or(&0);
            let right_max = right.iter().skip(1).max().unwrap_or(&0);
            let is_visible_lr = *left_max < height || *right_max < height;
            // println!("({}, {}) -> border: {}", r, c, is_border);
            // println!("({}, {}) -> lr visible: {}", r, c, is_visible_lr);
            let this_col = Vec::from_iter((0..=last_row).map(|r| grid[r][c]));
            let (above, below) = this_col.split_at(r);
            let above_max = above.iter().max().unwrap_or(&0);
            let below_max = below.iter().skip(1).max().unwrap_or(&0);
            let is_visible_ab = *above_max < height || *below_max < height;
            // println!("({}, {}) -> ab visible: {}", r, c, is_visible_ab);
            if is_border || is_visible_lr || is_visible_ab {
                visible_count += 1;
            }
        }
    }
    println!("visible count = {}", visible_count);
}
