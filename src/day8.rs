use regex::Regex;


fn get_dirs(line: &str)-> Vec<bool> {
    return line.chars().map(|c| c == 'R').collect();
}

fn get_dirs_keymap(contents: &String)->(Vec<bool>, Vec<(&str,&str,&str)>) {
    let mut keymap = Vec::new();
    let mut dirs = vec![];
    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    for (j,line) in contents.lines().enumerate() {
        if j == 0 {
            dirs = get_dirs(line);
            continue;
        }
        if j == 1 {
            continue;
        }
        let (_,[key,v1,v2]) = re.captures(line).unwrap().extract();
        keymap.push((key, v1, v2));
    }
    return (dirs, keymap);
}

fn find_path_len(dirs: &Vec<bool>, key_vec: &Vec<(usize,usize)>, ends_z: &Vec<bool>, start_idx: usize) -> usize {
    let mut key = start_idx;
    let mut dir_idx = 0;
    let mut len = 0;
    loop {
        if ends_z[key] {
            return len;
        }
        let dir = dirs[dir_idx];
        if dir {
            key = key_vec[key].1;
        } else {
            key = key_vec[key].0;
        }
        len += 1;
        dir_idx = (dir_idx + 1) % dirs.len();
    }
}

fn part1(dirs: Vec<bool>, keymap_unmut: Vec<(&str,&str,&str)>)->usize {
    let mut keymap = keymap_unmut.clone();
    keymap.sort_unstable_by_key(|k| k.0);
    let mut key_vec = Vec::new();
    let mut ends_z = Vec::new();
    for (key,v1,v2) in &keymap {
        let v1_idx = keymap.binary_search_by_key(v1, |&k| k.0).unwrap();
        let v2_idx = keymap.binary_search_by_key(v2, |&k| k.0).unwrap();
        let is_z = key == &"ZZZ";
        ends_z.push(is_z);
        key_vec.push((v1_idx, v2_idx));
    }
    return find_path_len(&dirs, &key_vec, &ends_z, 0);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn part2_helper(dirs: &Vec<bool>, key_vec: &Vec<(usize,usize)>, ends_z: &Vec<bool>, a_vec: &Vec<usize>)-> usize{
    let lens = a_vec.iter().map(|a| find_path_len(dirs, key_vec, ends_z, *a));
    let mut total_len = 1;
    for len in lens {
        let g = gcd(len, total_len);
        total_len *= len/g;
    }
    total_len
}

fn part2(dirs: Vec<bool>, keymap_unmut: Vec<(&str,&str,&str)>)->usize {
    let mut keymap = keymap_unmut.clone();
    keymap.sort_unstable_by_key(|k| k.0);
    let mut key_vec = Vec::new();
    let mut vec_a = Vec::new();
    let mut ends_z = Vec::new();
    for (j,(k,v1,v2)) in keymap.iter().enumerate() {
        let v1_idx = keymap.binary_search_by_key(v1, |&k| k.0).unwrap();
        let v2_idx = keymap.binary_search_by_key(v2, |&k| k.0).unwrap();
        key_vec.push((v1_idx, v2_idx));
        if k.chars().last().unwrap() == 'A' {
            vec_a.push(j);
        }
        let k_end_z = k.chars().last().unwrap() == 'Z';
        ends_z.push(k_end_z);
    }
    let sum = part2_helper(&dirs, &key_vec, &ends_z, &vec_a);
    return sum;
}

// idea: read in all the things into vector of (key, (v1, v2))
// sort by key and create map (key, idx)
// map the original vector into (idx, (left_idx, right_idx))
// map LRLL... or whatever into 0100...
pub fn day8(contents: &String) {
    let (dirs, keymap) = get_dirs_keymap(contents);
    let part1_ans = part1(dirs.clone(), keymap.clone());
    println!("Part 1: {part1_ans}");
    let part2_ans = part2(dirs, keymap);
    println!("Part 2: {part2_ans}");
}