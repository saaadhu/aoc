use std::fs;

fn basin_count (grid: &mut Vec<Vec<(u32, bool)>>, row: usize, col: usize, orig: u32) -> u32 {
    let mut count : u32 = 1;
    let elem = grid[row][col].0;
    let visited = grid[row][col].1;


    if visited || elem == 9 || elem < orig {
        //println! {"Hit boundary {}: {}", orig, elem};
        return 0
    }

    grid[row][col].1 = true;

    if row != 0 {
        count += basin_count (grid, row - 1, col, elem);
    }

    if col != 0 {
        count += basin_count (grid, row, col - 1, elem);
    }

    if row != grid.len() - 1 {
        count += basin_count (grid, row + 1, col, elem);
    }

    if col != grid[0].len() - 1 {
        count += basin_count (grid, row, col + 1, elem);
    }

    return count;
}

fn adjacents (grid: &mut Vec<Vec<(u32, bool)>>, row: usize, col: usize) -> Vec<u32> {
    let mut res : Vec<u32> = Vec::new();

    if row != 0 {
        res.push(grid[row - 1][col].0);
    }
    if col != 0 {
        res.push(grid[row][col - 1].0);
    }

    if row != grid.len() - 1 {
        res.push(grid[row+1][col].0);
    }

    if col != grid[0].len() - 1 {
        res.push(grid[row][col + 1].0);
    }

    return res;
}

fn main() {
    let input = fs::read_to_string("/home/saaadhu/scratch/input9").expect("Invalid file");

    let lines : Vec<&str> = input.split("\n").collect();
    let mut grid : Vec<Vec<(u32, bool)>> = Vec::new();

    for line in lines {
        let mut row : Vec<(u32, bool)> = Vec::new();
        for c in line.chars() {
            row.push((c.to_string().parse::<u32>().unwrap(), false));
        }
        grid.push(row);
    }

    let mut sizes: Vec<u32> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let elem = grid[i][j].0;
            let adj = adjacents(&mut grid, i,j);

            if adj.iter().all(|x| elem < *x) {
                sizes.push(basin_count(&mut grid, i, j, elem));
            }
        }
    }

    sizes.sort();
    let mut size = 1;
    for s in sizes.iter().rev().take(3) {
        size *= s;
    }

    println! {"Size is {}", size};
}
