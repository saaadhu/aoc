use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Fold {
    Horizontal(i32),
    Vertical(i32)
}

fn do_fold (dots: &mut HashSet<(i32, i32)>, fold: Fold) {
    let copy = dots.clone();
    for dot in copy {
        match fold {
            Fold::Horizontal (v) => {
                if dot.1 > v {
                    dots.remove(&dot);
                    dots.insert((dot.0, v - (dot.1 - v)));
                }
            }
            Fold::Vertical (v) => {
                if dot.0 > v {
                    dots.remove(&dot);
                    dots.insert((v - (dot.0 - v), dot.1));
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("/home/saaadhu/scratch/input13").expect("Invalid file");
    let lines : Vec<&str> = input.split("\n").collect();

    let mut dots : HashSet<(i32,i32)> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();

    for line in lines {
        if line.starts_with("fold") {
            let mut parts = line.split("=");
            let dir = parts.next().unwrap();
            let pt = parts.next().unwrap().parse::<i32>().unwrap();

            if dir.ends_with("y") {
                folds.push(Fold::Horizontal(pt));
            } else {
                folds.push(Fold::Vertical(pt));
            }
            continue;
        }
        if line == "" {
            continue;
        }

        let mut parts = line.split(",");
        dots.insert((parts.next().unwrap().parse::<i32>().unwrap(), parts.next().unwrap().parse::<i32>().unwrap()));
    }

    for fold in folds {
        do_fold (&mut dots, fold);
    }

    println! ("{:?}", dots);

    let mut max_x = -1; let mut max_y = -1;
    for dot in dots.iter() {
        if dot.0 > max_x {
            max_x = dot.0;
        }
        if dot.1 > max_y {
            max_y = dot.1;
        }
    }

    let mut grid :Vec<String> = vec![".".repeat((max_x + 1) as usize); (max_y + 1) as usize];

    for dot in dots.iter() {
        let pos = dot.0 as usize;
        let s: &mut String = &mut grid[dot.1 as usize];
        s.replace_range(pos..pos+1, "#");
    }

    for s in grid {
        println! {"{}", s};
    }
}

      
