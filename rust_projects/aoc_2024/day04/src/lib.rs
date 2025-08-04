// #[allow(unused_variables)]
// #[allow(unused_mut)]
// #[allow(dead_code)]
// println!("{:#?}", var);
pub fn day4_part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let xmas_total = search_xmas(grid);
    println!("{}", xmas_total);
    10
}

fn search_xmas(grid: Vec<Vec<char>>) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let target = ['X', 'M', 'A', 'S'];
    let directions: &[(isize, isize)] = &[
        (0, 1),   // right →
        (0, -1),  // left ←
        (1, 0),   // down ↓
        (-1, 0),  // up ↑
        (1, 1),   // diagonal down-right ↘
        (1, -1),  // diagonal down-left ↙
        (-1, 1),  // diagonal up-right ↗
        (-1, -1), // diagonal up-left ↖
    ];

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in directions {
                let mut found = true;

                for i in 0..4 {
                    let r = row as isize + dr * i as isize;
                    let c = col as isize + dc * i as isize;

                    if r < 0 || r >= rows as isize || c < 0 || c >= cols as isize {
                        found = false;
                        break;
                    }

                    if grid[r as usize][c as usize] != target[i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }
    }
    count
}
