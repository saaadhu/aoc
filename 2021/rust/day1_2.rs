use std::fs;

fn add_to_window (window: &mut Vec<i32>, v:i32) {
    if window_full(window) {
        window.pop();
    }
    window.insert(0, v);
}

fn window_full(window: &Vec<i32>) -> bool {
  return window.len() == 3;
}

fn window_sum(window: &Vec<i32>) -> i32 {
  let mut sum = 0;
  for e in window {
      sum += e;
  }
  return sum;
}

fn main() {
    let contents = fs::read_to_string("/home/saaadhu/scratch/input")
        .expect("Could not read file");

    let mut increases = 0;
    let mut previous = -1;

    let mut w: Vec<i32> = Vec::new();

    for s in contents.split_whitespace() {
        add_to_window (&mut w, s.parse().expect("Could not parse number"));
        if !window_full(&w) {
            continue;
        }
        let current = window_sum(&w);
        let a = w.get(0).expect("Not enough"); let b = w.get(1).expect("Not enough"); let c = w.get(2).expect("Not enough elements");
        println!("[{},{},{}] = {}",a,b,c,current);
        if previous != -1 && current > previous {
            increases = increases + 1;
        }
        previous = current;
    }
    
    println!("Increases: {}", increases);
}
