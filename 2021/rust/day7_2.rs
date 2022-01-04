use std::fs;

fn main() {
    let positions : Vec<i64> = fs::read_to_string("/home/saaadhu/scratch/input7")
        .expect("Invalid file")
        .split(",")
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    let mut fuel: Vec<i64> = Vec::new();

    for i in 0..positions.len() {
        fuel.push(0);
        for j in 0..positions.len() {
            let diff = (positions[i] - positions[j]).abs();
            fuel[i] += (diff * (diff + 1))/2;
        }
    }

    println! ("{}", fuel.iter().min().unwrap());
}
