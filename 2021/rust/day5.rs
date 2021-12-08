use std::fs;
use std::cmp;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

fn parse_coord(s: &str) -> Point {
    let parts: Vec<&str> = s.trim().split(",").collect();
    return Point {x: parts[0].parse::<usize>().unwrap(),
                  y: parts[1].parse::<usize>().unwrap() };
}

fn parse_line(s: &str) -> Line {
    let mut parts = s.split("->");
    return Line { start: parse_coord(parts.next().unwrap()),
                  end: parse_coord(parts.next().unwrap())
    };
}

fn horizontal (a: &Line) -> bool {
    return a.start.y == a.end.y;
}

fn vertical (a: &Line) -> bool {
    return a.start.x == a.end.x;
}

fn diagonal (a: &Line) -> bool {
    return i32::abs(a.end.y as i32 - a.start.y as i32) == i32::abs(a.end.x as i32 - a.start.x as i32);
}

fn main() {
    let lines : Vec<Line> = fs::read_to_string("/home/saaadhu/scratch/input5")
        .expect("Invalid file")
        .lines()
        .map(parse_line)
        .collect();

    let mut grid = [[0; 1000]; 1000];

    for line in lines {
        let x1 = cmp::min(line.start.x, line.end.x);
        let x2 = cmp::max(line.start.x, line.end.x);

        let y1 = cmp::min(line.start.y, line.end.y);
        let y2 = cmp::max(line.start.y, line.end.y);

        println! ("Considering line {:?}", line);
        // Horizontal line
        if horizontal(&line) {
            let mut x = x1;
            while x <= x2 {
                grid[x][y1] += 1;

//              println! ("Bumping {},{} to {}", x, y1, grid[x][y1]);
                x += 1;
            }
        }
        else if vertical(&line) {
            let mut y = y1;
            // Vertical line
            while y <= y2 {
                grid[x1][y] += 1;
 //               println! ("Bumping {},{} to {}", x1, y, grid[x1][y]);
                y += 1;
            }
        }
        else if diagonal(&line) {
            let mut x = x1;
            let mut y = if x1 == line.start.x {line.start.y} else {line.end.y};
            let decy = y == y2;

            while x <= x2 {
                grid[x][y] += 1;

                x += 1; y = if decy && y > 0 {y - 1} else {y + 1};
            }
        }
    }


    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] >= 2 {
                count += 1;
            }
        }
    }
}
