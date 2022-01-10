use std::fs;

fn adjacent_positions(row : i32, col: i32) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();
    let all = [(row - 1, col), (row - 1, col + 1), (row, col + 1), (row + 1, col + 1), (row + 1, col), (row + 1, col - 1), (row, col - 1), (row - 1, col - 1)];

    for (x,y) in all {
        if x < 0 || y < 0 || x > 9 || y > 9 {
            continue;
        }
        positions.push((x as usize, y as usize));
    }
    return positions;
}

fn flash(grid: &mut Vec<Vec<(u32, bool)>>, input_pos: (usize,usize)) -> i32 {
    let row = input_pos.0; let col = input_pos.1;
    
    if grid[row][col].1 == true {
        return 0;
    }

    let mut flashes = 1;
    grid[row][col].1 = true;

    let positions = adjacent_positions(row as i32, col as i32);
    for pos in positions.iter() {
        grid[pos.0][pos.1].0 += 1;

        if grid[pos.0][pos.1].0 > 9 {
            flashes += flash (grid, (pos.0, pos.1));
        }
    }

    return flashes;
}

fn do_step(grid: &mut Vec<Vec<(u32, bool)>>) -> i32 {
    let mut flashes = 0;

    let bounds = 10;

    for row in 0..bounds {
        for col in 0..bounds {
            grid[row][col].0 += 1;
        }
    }

    for row in 0..bounds {
        for col in 0..bounds {
            if grid[row][col].0 > 9 {
                flashes += flash (grid, (row,col))
            }
        }
    }

    for row in 0..bounds {
        for col in 0..bounds {
            if grid[row][col].1 == true {
                grid[row][col] = (0, false);
            }
        }
    }
    return flashes;
}

fn main() {
    let input = fs::read_to_string("/home/saaadhu/scratch/input11").expect("Invalid file");

    let lines : Vec<&str> = input.split("\n").collect();

    let mut grid: Vec<Vec<(u32, bool)>> = Vec::new();

    for line in lines {
        let mut row : Vec<(u32, bool)> = Vec::new();
        for c in line.chars() {
            row.push((c.to_string().parse::<u32>().unwrap(), false));
        }
        grid.push(row);
    }


    for i in 0.. {
        do_step(&mut grid);

        if grid.iter().all(|v| v.iter().all(| (strength, _)| strength == &0)) {
            println! ("{}", i + 1);
            break;
        }

    }
}

      
