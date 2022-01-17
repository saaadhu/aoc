use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash)]
struct Cave<'a> {
    name: &'a str,
    visited : bool,
    small: bool,
    adj: Vec<&'a str>
}

impl<'a> PartialEq for Cave<'a> {
    fn eq (self: &Self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn find_paths<'a> (current : &str, start : &str, end: &str, exception: &str, nodemap: &'a HashMap<&str, Cave<'a>>, visited: &'a mut HashMap<&'a Cave<'a>, i32>) -> i32 {
    if current == end {
        return 1;
    }

    let current_node = nodemap.get(current).unwrap();
    let count = visited.get(current_node).or(Some(&0)).unwrap();

    if current_node.small == true {
        if current == exception && count >= &2 {
            return 0;
        }

        if current != exception && count >= &1 {
            return 0;
        }
    }

    let mut newvisited = visited.clone();
    newvisited.insert(current_node, count+1);

    let mut paths = 0;
    for a in current_node.adj.iter() {
        if *a == start {
            continue;
        }
        let path = find_paths(a, start, end, exception, nodemap, &mut newvisited.clone());
        if path != 0 {
            paths += path;
        }
    }

    return paths;
}

fn create_cave (name: &str) -> Cave {
        Cave {
            name,
            visited: false,
            small: name.chars().all(|x| x.is_ascii_lowercase()),
            adj: vec![]
        }
}

fn main() {
    let input = fs::read_to_string("/home/saaadhu/scratch/input12").expect("Invalid file");

    let lines : Vec<&str> = input.split("\n").collect();

    let mut nodemap : HashMap<&str, Cave> = HashMap::new();

    let mut smallcaves : HashSet<&str> = HashSet::new();
    for line in lines.iter() {
        let mut parts = line.split("-");
        let srcname = parts.next().unwrap();
        let dstname = parts.next().unwrap();

        nodemap.entry(dstname).or_insert(create_cave(dstname)).adj.push(srcname);
        nodemap.entry(srcname).or_insert(create_cave(srcname)).adj.push(dstname);

        if srcname.chars().all(|x| x.is_ascii_lowercase()) {
            smallcaves.insert(srcname);
        }

        if dstname.chars().all(|x| x.is_ascii_lowercase()) {
            smallcaves.insert(dstname);
        }
    }

    smallcaves.remove("start");
    smallcaves.remove("end");

    let single_paths = find_paths("start", "start", "end", "", &nodemap, &mut HashMap::new());
    let mut paths = single_paths;

    for cave in smallcaves {
        paths += find_paths("start", "start", "end", cave, &nodemap, &mut HashMap::new());
        paths -= single_paths;
    }

    println! ("{}", paths);
}

      
