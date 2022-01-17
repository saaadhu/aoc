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

fn find_paths<'a> (current : &str, start : &str, end: &str, nodemap: &'a HashMap<&str, Cave<'a>>, visited: &'a mut HashSet<&'a Cave<'a>>) -> i32 {
    if current == end {
        return 1;
    }

    let current_node = nodemap.get(current).unwrap();

    if visited.contains(current_node) && current_node.small == true {
        return 0;
    }

    let mut newvisited = visited.clone();
    newvisited.insert(current_node);

    let mut paths = 0;
    for a in current_node.adj.iter() {
        if *a == start {
            continue;
        }
        let path = find_paths(a, start, end, nodemap, &mut newvisited.clone());
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

    for line in lines.iter() {
        let mut parts = line.split("-");
        let srcname = parts.next().unwrap();
        let dstname = parts.next().unwrap();

        nodemap.entry(dstname).or_insert(create_cave(dstname)).adj.push(srcname);
        nodemap.entry(srcname).or_insert(create_cave(srcname)).adj.push(dstname);
    }

    let paths = find_paths("start", "start", "end", &nodemap, &mut HashSet::new());

    println! ("{}", paths);
}

      
