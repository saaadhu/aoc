use std::fs;


#[derive(Debug, Clone)]
struct BoardPosition {
    num :u32,
    marked: bool
}

type Board = Vec<Vec<BoardPosition>>;

fn main() {
    let mut numbers : Vec<u32> = Vec::new();
    let mut boards : Vec<Board> = Vec::new();
    
    let contents = fs::read_to_string("/home/saaadhu/scratch/input4").expect("Invalid file");
    let mut iter = contents.split("\n");
    let numbersline = iter.next().unwrap();
    for number in numbersline.split(",") {
        numbers.push(number.parse().unwrap());
    }
    iter.next();

    let mut board : Vec<Vec<BoardPosition>> = Vec::new();
    for line in iter {
        if line == "" {
            boards.push(board.clone());
            board.clear();
            continue;
        }

        let mut row : Vec<BoardPosition> = Vec::new();
        for part in line.split_whitespace() {
            let num : u32 = part.parse().unwrap();
            row.push(BoardPosition {num : num, marked: false});
        }
        board.push(row.clone());
    }

    boards.push(board.clone());

    let mut lastscore = 0;
    let mut deletedboards : Vec<usize> = Vec::new();

    for num in numbers {
        println! ("Choosing {}", num);

        for i in 0..boards.len() {
            if deletedboards.contains(&i) {
                continue;
            }
            for row in 0..board.len() {
                for col in 0..board.len() {
                    if boards[i][row][col].num == num {
                        boards[i][row][col].marked = true;
                    }
                }
            }
        }

       'outer:for i in 0..boards.len() {
            if deletedboards.contains(&i) {
                continue;
            }
            for row in 0..board.len() {
                let mut rowmarked = true;
                for col in 0..board.len() {
                    if !boards[i][row][col].marked {
                        rowmarked = false;
                        break;
                    }
                }

                if rowmarked {
                    println! ("Found row {} marked of board {}", row, i);
                    let mut unmarkedsum = 0;

                    for row in 0..board.len() {
                        for col in 0..board.len() {
                            if !boards[i][row][col].marked {
                                unmarkedsum += boards[i][row][col].num;
                            }
                        }
                    }
                    println! ("{} * {} = {}", unmarkedsum, num, unmarkedsum * num);
                    if unmarkedsum == 0 {
                        println! ("{:?}", boards[i]);
                    }
                    lastscore = unmarkedsum * num;
                    deletedboards.push(i); continue 'outer;
                }
            }
        }

        'outer2:for i in 0..boards.len() {
            if deletedboards.contains(&i) {
                continue;
            }
            for col in 0..board.len() {
                let mut colmarked = true;
                for row in 0..board.len() {
                    if !boards[i][row][col].marked {
                        colmarked = false;
                        break;
                    }
                }

                if colmarked {
                    println! ("Found col {} marked of board {}", col, i);

                    let mut unmarkedsum = 0;
                    for row in 0..board.len() {
                        for col in 0..board.len() {
                            if !boards[i][row][col].marked {
                                unmarkedsum += boards[i][row][col].num;
                            }
                        }
                    }
                    println! ("{} * {} = {}", unmarkedsum, num, unmarkedsum * num);
                    if unmarkedsum == 0 {
                        println! ("{:?}", boards[i]);
                    }

                    lastscore = unmarkedsum * num;
                    deletedboards.push(i); continue 'outer2;
                }
            }
        }
    }

    println! ("{}", lastscore);
}


