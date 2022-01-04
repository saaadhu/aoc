use std::fs;

fn main() {
    let mut positions : Vec<i64> = fs::read_to_string("/home/saaadhu/scratch/input7")
        .expect("Invalid file")
        .split(",")
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    positions.sort();

    let min_fuel_posn = positions[positions.len()/2];

    let min_fuel: i64 = positions
        .iter()
        .map(|p| (min_fuel_posn - p).abs())
        .sum();

    println! ("{}", min_fuel);
}
