fn main() {
    // let grid_str = highlight((3,4,-1,0)); // ↑
    // let grid_str = highlight((0,4,1,0)); // ↓
    // let grid_str = highlight((0,4,0,-1)); // ←
    // let grid_str = highlight((0,4,0,1)); // →
    // let grid_str = highlight((0,4,1,1)); // ↘
    let grid_str = highlight((0,4,1,-1)); // ↙
    // let grid_str = highlight((3,4,-1,1)); // ↗
    // let grid_str = highlight((3,4,-1,-1)); // ↖
    
    println!("{}", grid_str);
    // dbg!(grid_str);
}

// #[allow(unused_mut)]
// #[allow(unused_variables)]
// Convert coordinates to string matrix
fn highlight((x, y, dx, dy): (i8,i8,i8,i8)) -> String {
    let input = vec!["*".repeat(10); 10].join("\n");

    dbg!(x, y, dx, dy);

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // 0, 1 → | 0, -1 ← | 1, 0 ↓ | -1, 0 ↑ | 1, 1 ↘ | 1, -1 ↙ | -1, 1 ↗ | -1, -1 ↖
    
    let letters = ['A', 'B', 'C', 'D'];

    for (i, ch) in letters.iter().enumerate() {
        let xi = x + dx * i as i8;
        let yi = y + dy * i as i8;

        // Bounds checking can be added here if needed
        grid[xi as usize][yi as usize] = *ch;
    }
    /* if dx == 0 && dy == 1 { // →
        grid[x as usize][y as usize] = 'A';
        grid[(x) as usize][(y+1) as usize] = 'B';
        grid[(x) as usize][(y+2) as usize] = 'C';
        grid[(x) as usize][(y+3) as usize] = 'D';
    } else if dx == 0 && dy == -1 { // ←
        grid[x as usize][y as usize] = 'A';
        grid[(x) as usize][(y-1) as usize] = 'B';
        grid[(x) as usize][(y-2) as usize] = 'C';
        grid[(x) as usize][(y-3) as usize] = 'D';
    } else if dx == 1 && dy == 0 { // ↓
        grid[x as usize][y as usize] = 'A';
        grid[(x+1) as usize][(y) as usize] = 'B';
        grid[(x+2) as usize][(y) as usize] = 'C';
        grid[(x+3) as usize][(y) as usize] = 'D';
    } else if dx == -1 && dy == 0 { // ↑
        grid[x as usize][y as usize] = 'A';
        grid[(x-1) as usize][(y) as usize] = 'B';
        grid[(x-2) as usize][(y) as usize] = 'C';
        grid[(x-3) as usize][(y) as usize] = 'D';
    } else if dx == 1 && dy == 1 { // ↘
        grid[x as usize][y as usize] = 'A';
        grid[(x+1) as usize][(y+1) as usize] = 'B';
        grid[(x+2) as usize][(y+2) as usize] = 'C';
        grid[(x+3) as usize][(y+3) as usize] = 'D';
    } else if dx == 1 && dy == -1 { // ↙
        grid[x as usize][y as usize] = 'A';
        grid[(x+1) as usize][(y-1) as usize] = 'B';
        grid[(x+2) as usize][(y-2) as usize] = 'C';
        grid[(x+3) as usize][(y-3) as usize] = 'D';
    } else if dx == -1 && dy == 1 { // ↗
        grid[x as usize][y as usize] = 'A';
        grid[(x-1) as usize][(y+1) as usize] = 'B';
        grid[(x-2) as usize][(y+2) as usize] = 'C';
        grid[(x-3) as usize][(y+3) as usize] = 'D';
    } else if dx == -1 && dy == -1 { // ↖
        grid[x as usize][y as usize] = 'A';
        grid[(x-1) as usize][(y-1) as usize] = 'B';
        grid[(x-2) as usize][(y-2) as usize] = 'C';
        grid[(x-3) as usize][(y-3) as usize] = 'D';
    } */


    let grid_str = grid
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    grid_str
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = highlight((0,4,0,1)); // →
        assert_eq!(result, "****ABCD**
**********
**********
**********
**********
**********
**********
**********
**********
**********");
    }

    #[test]
    fn t2() {
        let result = highlight((0,4,0,-1)); // ←
        assert_eq!(result, "*DCBA*****
**********
**********
**********
**********
**********
**********
**********
**********
**********");
    }

    #[test]
    fn t3() {
        let result = highlight((0,4,1,0)); // ↓
        assert_eq!(result, "****A*****
****B*****
****C*****
****D*****
**********
**********
**********
**********
**********
**********");
    }

    #[test]
    fn t4() {
        let result = highlight((3,4,-1,0)); // ↑
        assert_eq!(result, "****D*****
****C*****
****B*****
****A*****
**********
**********
**********
**********
**********
**********");
    }

    #[test]
    fn t5() {
        let result = highlight((0,4,1,1)); // ↘
        assert_eq!(result, "****A*****
*****B****
******C***
*******D**
**********
**********
**********
**********
**********
**********");
    }

    #[test]
    fn t6() {
        let result = highlight((0,4,1,-1)); // ↙
        assert_eq!(result, "****A*****
***B******
**C*******
*D********
**********
**********
**********
**********
**********
**********");
    }

    #[test]
    fn t7() {
        let result = highlight((3,4,-1,1)); // ↗
        assert_eq!(result, "*******D**
******C***
*****B****
****A*****
**********
**********
**********
**********
**********
**********");
    }

    #[test]
    fn t8() {
        let result = highlight((3,4,-1,-1)); // ↖
        assert_eq!(result, "*D********
**C*******
***B******
****A*****
**********
**********
**********
**********
**********
**********");
    }

}


