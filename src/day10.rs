use itertools::Itertools;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Dir {S, E, N, W, NONE, START}

fn get_dir(end1: Dir, end2: Dir, from: Dir)->Dir {
    if from == end1 {
        return end2;
    }
    if from == end2 {
        return end1;
    }
    return Dir::NONE;
}

fn next_dir_idx(to: Dir, from_idx: (i64,i64))->(Dir,(i64,i64)) {
    match to {
        Dir::N=>return (Dir::S, (from_idx.0-1, from_idx.1)),
        Dir::S=>return (Dir::N, (from_idx.0+1, from_idx.1)),
        Dir::E=>return (Dir::W, (from_idx.0, from_idx.1+1)),
        Dir::W=>return (Dir::E, (from_idx.0, from_idx.1-1)),
        _=> return (Dir::NONE, (-1, -1))
    }
}

fn map_dir(c:char, from:Dir)->Dir {
    match c {
        '|'=>return get_dir(Dir::N, Dir::S, from),
        '-'=>return get_dir(Dir::E, Dir::W, from),
        'L'=>return get_dir(Dir::N, Dir::E, from),
        'J'=>return get_dir(Dir::N, Dir::W, from),
        '7'=>return get_dir(Dir::S, Dir::W, from),
        'F'=>return get_dir(Dir::S, Dir::E, from),
        'S'=>return Dir::START,
         _ =>return Dir::NONE
    }
}

fn get_inc(out_dir: Dir, idx: (i64, i64))->i64 {
    let d = out_dir as i8;
    let sign = if d<2 {1} else {-1};
    let ret = if d % 2 == 0 {sign*idx.1} else {sign*idx.0};
    println!("idx:({},{}), {:?}, ret:{}", idx.0, idx.1, out_dir, ret);
    return ret;
}

fn search(map: &Vec<Vec<char>>, start_idx:(i64, i64), start_from: Dir)->(i64,i64) {
    let mut idx = start_idx;
    let mut from = start_from;
    let mut dist = 1;
    let mut area_int = 0;
    loop {
        if idx.0 < 0 || idx.1 < 0 || idx.0 as usize >= map.len() || idx.1 as usize >= map[0].len() {
            return (-1,-1);
        }
        let (row, col) = (idx.0 as usize, idx.1 as usize);
        let to = map_dir(map[row][col], from);
        if to == Dir::NONE {
            return (-1,-1);
        }
        if to == Dir::START {
            return (dist / 2, area_int / 2);
        }
        area_int += get_inc(to, idx);
        dist += 1;
        (from, idx) = next_dir_idx(to, idx);
    }
}

pub fn day10(contents: &String) {
    let mut s_idx = (-1,-1);
    let mut map: Vec<Vec<char>> = Vec::new();
    for (i,line) in contents.lines().enumerate() {
        let line_chars = line.chars().collect_vec();
        if s_idx == (-1, -1) {
            for (j, c) in line_chars.iter().enumerate() {
                if c == &'S' {
                    s_idx = (i as i64,j as i64);
                }
            }
        }
        map.push(line_chars);
    }
    for to in [Dir::N, Dir::S, Dir::E, Dir::W] {
        let (start_from, start_idx) = next_dir_idx(to, s_idx);
        let (dist, area) = search(&map, start_idx, start_from);
        if dist > 0 {
            let pick = area - dist + 1;
            println!("Part 1: {dist}");
            println!("Part 2: {pick}");
            break;
        }
    }
}