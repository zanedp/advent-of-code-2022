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
    let mut max_scenic_score = 0;
    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let height = grid[r][c];
            let is_border = (r == 0) || (c == 0) || (r == last_row) || (c == last_col);
            let this_row = &grid[r];
            let (west, east) = this_row.split_at(c);
            let left_max = west.iter().max().unwrap_or(&0);
            let right_max = east.iter().skip(1).max().unwrap_or(&0);
            let is_visible_lr = *left_max < height || *right_max < height;
            // println!("({}, {}) -> border: {}", r, c, is_border);
            // println!("({}, {}) -> lr visible: {}", r, c, is_visible_lr);
            let this_col = Vec::from_iter((0..=last_row).map(|r| grid[r][c]));
            let (north, south) = this_col.split_at(r);
            let above_max = north.iter().max().unwrap_or(&0);
            let below_max = south.iter().skip(1).max().unwrap_or(&0);
            let is_visible_ab = *above_max < height || *below_max < height;
            // println!("({}, {}) -> ab visible: {}", r, c, is_visible_ab);

            let is_visible = is_border || is_visible_lr || is_visible_ab;
            if is_visible {
                visible_count += 1;
            }

            let north_view: Vec<i8> = north.iter().rev().cloned().collect();
            let east_view: Vec<i8> = east.iter().skip(1).cloned().collect();
            let south_view: Vec<i8> = south.iter().skip(1).cloned().collect();
            let west_view: Vec<i8> = west.iter().rev().cloned().collect();

            // println!("-- ({}, {}) --", r, c);
            let north_view_dist = find_viewing_dist(height, &north_view);
            let east_view_dist = find_viewing_dist(height, &east_view);
            let south_view_dist = find_viewing_dist(height, &south_view);
            let west_view_dist = find_viewing_dist(height, &west_view);
            let scenic_score = north_view_dist * east_view_dist * south_view_dist * west_view_dist;
            // if r == 3 && c == 2 {
            // println!("scenic score: {}", scenic_score);
            // }
            max_scenic_score = max_scenic_score.max(scenic_score);
        }
    }
    println!("visible count = {}", visible_count);
    println!("max scenic score = {}", max_scenic_score);
}

fn find_viewing_dist(tree_house_height: i8, eye_line: &Vec<i8>) -> usize {
    let mut found_edge = false;
    let in_view: Vec<_> = eye_line
        .iter()
        .take_while(|cur| {
            if found_edge {
                return false;
            }
            let is_stop = **cur >= tree_house_height;
            found_edge |= is_stop;
            true
        })
        .collect();
    // println!("{:?}", in_view);
    in_view.len()
}

#[test]
fn test_find_viewing_distance() {
    let line = vec![0i8, 3, 5, 5, 3, 7, 9, 3];
    let viewing_distance = find_viewing_dist(6, &line);
    println!("viewing distance: {}", viewing_distance);
}
