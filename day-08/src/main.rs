fn print_grid(grid: &[Vec<i8>]) {
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
    let mut max_scenic_score = 0;
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
            let (up, down) = this_col.split_at(r);
            let up_max = up.iter().max().unwrap_or(&0);
            let down_max = down.iter().skip(1).max().unwrap_or(&0);
            let is_visible_ud = *up_max < height || *down_max < height;
            // println!("({}, {}) -> ab visible: {}", r, c, is_visible_ab);

            let is_visible = is_border || is_visible_lr || is_visible_ud;
            if is_visible {
                visible_count += 1;
            }

            let up_view: Vec<i8> = up.iter().rev().cloned().collect();
            let right_view: Vec<i8> = right.iter().skip(1).cloned().collect();
            let down_view: Vec<i8> = down.iter().skip(1).cloned().collect();
            let left_view: Vec<i8> = left.iter().rev().cloned().collect();

            // println!("-- ({}, {}) --", r, c);
            let up_view_dist = find_viewing_dist(height, &up_view);
            let right_view_dist = find_viewing_dist(height, &right_view);
            let down_view_dist = find_viewing_dist(height, &down_view);
            let left_view_dist = find_viewing_dist(height, &left_view);
            let scenic_score = up_view_dist * right_view_dist * down_view_dist * left_view_dist;
            // if r == 3 && c == 2 {
            // println!("scenic score: {}", scenic_score);
            // }
            max_scenic_score = max_scenic_score.max(scenic_score);
        }
    }
    println!("visible count = {}", visible_count);
    println!("max scenic score = {}", max_scenic_score);
}

fn find_viewing_dist(tree_house_height: i8, eye_line: &[i8]) -> usize {
    let mut found_edge = false;
    eye_line
        .iter()
        .take_while(|cur| {
            if found_edge {
                return false;
            }
            let is_stop = **cur >= tree_house_height;
            found_edge |= is_stop;
            true
        })
        .count()
}

#[test]
fn test_find_viewing_distance() {
    let line = vec![0i8, 3, 5, 5, 3, 7, 9, 3];
    let viewing_distance = find_viewing_dist(6, &line);
    println!("viewing distance: {}", viewing_distance);
}
