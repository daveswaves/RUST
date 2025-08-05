// #[allow(unused_variables)]
// #[allow(unused_mut)]
// #[allow(dead_code)]
// println!("{:#?}", var);
pub fn day4_part1(input: &str) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // let xmas_total = search_xmas(grid);
    // println!("{}", xmas_total);

    let (count, matches) = search_xmas(grid);
    
    println!("Found {} occurrences of 'XMAS'", count);
    for (row, col, _dr, _dc) in matches {
        // let grid_str = highlight((row, col, dr, dc));
        
        println!("Match at ({}, {}):", row, col);
        // println!("Match at ({}, {}) going direction ({}, {})", row, col, dr, dc);
        // println!("{}", grid_str);
    }
}

fn search_xmas(grid: Vec<Vec<char>>) -> (u32, Vec<(usize, usize, isize, isize)>) {
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
    let mut matches = Vec::new(); // For highlighing

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
                    matches.push((row, col, dr, dc));
                }
            }
        }
    }
    (count, matches)
}

pub fn highlight((x, y, dx, dy): (usize, usize, isize, isize)) -> String {
    let input = vec!["*".repeat(10); 10].join("\n");

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let letters = ['X', 'M', 'A', 'S'];
    // let letters = ['1', '2', '3', '4'];

    for (i, ch) in letters.iter().enumerate() {
        let xi = x as isize + dx * i as isize;
        let yi = y as isize + dy * i as isize;

        // grid[xi as usize][yi as usize] = *ch;
        // Bounds check to avoid panic on negative or overflow
        if xi >= 0 && yi >= 0 &&
           (xi as usize) < grid.len() &&
           (yi as usize) < grid[0].len() {
            grid[xi as usize][yi as usize] = *ch;
        }
    }

    let grid_str = grid
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    grid_str
}
