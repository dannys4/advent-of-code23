
fn min_max_race(time: u64, dist: u64) -> u64 {
    let mut below = 0;
    let mut above = time;
    let mut find_below = true;
    let mut find_above = true;
    while find_above && find_below {
        if find_below {
            if (time-below)*below > dist {
                find_below = false;
            } else {
                below += 1;
            }
        }
        if find_above {
            if (time-above)*above <= dist {
                above -= 1;
            } else {
                find_above = false;
            }
        }
    }
    while find_below {
        if (time-below)*below > dist {
            find_below = false;
        } else {
            below += 1;
        }
    }
    while find_above {
        if (time-above)*above <= dist {
            above -= 1;
        } else {
            find_above = false;
        }
    }
    let ret = 1+above-below;
    return ret;
}

pub fn day6(contents: &String) {
    let lines = contents.rsplit_once('\n').unwrap();
    let mut time_dist = Vec::new();
    let mut should_pass = true;
    for (time, dist) in lines.0.split_ascii_whitespace().zip(lines.1.split_ascii_whitespace()) { //re.captures_iter(lines.0).zip(re.captures_iter(lines.1)) {
        if should_pass {
            should_pass = false;
            continue;
        }
        time_dist.push((time.parse::<u64>().unwrap(), dist.parse::<u64>().unwrap()));
    }
    let res:u64 = time_dist.iter().map(|t| min_max_race(t.0, t.1)).product();
    println!("Part 1: {res}");
    let mut time = 0;
    let mut dist = 0;
    let pow10 = |x| (10 as f64).powf((x as f64).log10().ceil()) as u64;
    for (t, d) in time_dist {
        let t_power = pow10(t);
        let d_power = pow10(d);
        time = time*t_power + t;
        dist = dist*d_power + d;
    }
    let res2 = min_max_race(time, dist);
    println!("Part 2: {res2}");
}