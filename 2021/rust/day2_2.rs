use std::fs;

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32)
}

fn parse_command (s: String) -> Direction {
   println!("{}", s);
   let mut parts = s.split_whitespace();
   let dir = parts.next().unwrap();
   let value = parts.next().unwrap().parse().expect("Expected number");
   return match dir {
       "forward" => Direction::Forward(value),
       "up" => Direction::Up(value),
       "down" => Direction::Down(value),
       _ => panic! ("Unexpected direction")
    }
}

fn main() {
    let contents = fs::read_to_string("/home/saaadhu/scratch/input2")
        .expect("Could not read file");

    let mut forward = 0;
    let mut aim = 0;
    let mut depth = 0;

    for s in contents.split("\n") {
        match parse_command(s.to_string()) {
              Direction::Forward(x) => {
                  forward += x;
                  depth += aim * x;
              }
              Direction::Up(x) => aim -= x,
              Direction::Down(x) => aim += x
        }
    }
    
    println!("Result: {}", forward * depth);
}
