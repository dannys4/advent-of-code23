use core::i64::MAX;
use std::{cmp::min, time::Instant};
use regex::Regex;
// use indicatif::{ParallelProgressIterator, ProgressStyle, ProgressBar};
use rayon::prelude::*;

fn make_map(map_str: &str)->Vec<(i64, i64, i64)> {
    let mut mymap = Vec::new();
    for line in map_str.lines() {
        let nums: Vec<i64> = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let start = *nums.get(1).unwrap();
        let end = start + *nums.get(2).unwrap() - 1;
        let offset = *nums.get(0).unwrap() - start;
        mymap.push((start, end, offset));
    }
    return mymap;
}

fn find_dest(seed: i64, maps: &Vec<Vec<(i64,i64,i64)>>)->i64 {
    let mut curr_val = seed;
    for map in maps {
        let mut idx = 0;
        while idx < map.len() {
            let map_idx = map.get(idx).unwrap();
            if map_idx.0 <= curr_val && map_idx.1 >= curr_val {
                curr_val += map_idx.2;
                break;
            }
            idx += 1;
        }
    }
    return curr_val;
}

fn find_min_location(seeds: &Vec<i64>, maps: &Vec<Vec<(i64, i64, i64)>>)->i64 {
    let mut min_loc = MAX;
    let mut _min_seed = 0;
    for seed in seeds {
        let dest = find_dest(*seed, maps);
        if dest < min_loc {
            min_loc = dest;
            _min_seed = *seed;
        }
    }
    return min_loc;
}

pub fn day5(contents: &String) {
    let mut maps = Vec::new();
    let mut seeds = Vec::new();
    let mut seed_str= "";
    for (j, full_map_str) in contents.split("\n\n").enumerate() {
        if j == 0 {
            seed_str = full_map_str.rsplit_once(": ").unwrap().1;
            for seed in seed_str.split_ascii_whitespace() {
                seeds.push(seed.parse::<i64>().unwrap());
            }
        } else {
            let map_str = full_map_str.rsplit_once(":\n").unwrap().1;
            let mut map_j = make_map(map_str);
            map_j.sort_by_key(|k| k.0);
            maps.push(map_j);
        }
    }
    let min_loc = find_min_location(&seeds, &maps);
    println!("Part 1: {min_loc}");

    seeds.clear();
    let re = Regex::new(r"([0-9]+) ([0-9]+)").unwrap();
    let mut min_dest = MAX;
    let mut seed_vec = Vec::new();
    let mut _total = 0;
    for c in re.captures_iter(seed_str) {
        let (_, [seed_start_str, range_str]) = c.extract();
        let seed_start = seed_start_str.parse::<i64>().unwrap();
        let range = range_str.parse::<i64>().unwrap();
        seed_vec.push((seed_start, range));
        _total += range as u64;
    }
    
    let start = Instant::now();
    // let bar = ProgressBar::new(_total).with_style(style);
    for (seed_start, range) in &seed_vec {
        // let style = ProgressStyle::with_template("[{eta}] {bar:40.cyan/blue}").unwrap();
        let seed_end = *seed_start + *range;
        let my_range: Vec<_> = (*seed_start..seed_end).collect();
        let range_min = my_range.par_iter()//.progress_with_style(style)//progress_with_style(bar)
            .map( |seed| find_dest(*seed, &maps)).min().unwrap();
        min_dest = min(min_dest, range_min);
    }
    let elap = start.elapsed();

    println!("Part 2: {min_dest}, {}s", elap.as_secs());
}