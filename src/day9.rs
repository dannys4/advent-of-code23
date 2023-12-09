use std::mem::swap;

fn create_values(vec: &Vec<i64>)->(i64,i64) {
    let mut last_vals = vec![*vec.last().unwrap()];
    let mut first_vals = vec![vec[0]];
    let mut old_vec = vec.clone();
    let mut new_vec = vec.clone();
    let mut n_diff = vec.len() - 1;
    loop {
        let mut all_zeros = true;
        for j in 0..n_diff {
            new_vec[j] = old_vec[j+1] - old_vec[j];
            all_zeros &= new_vec[j] == 0;
        }
        if all_zeros {
            break;
        }
        last_vals.push(new_vec[n_diff-1]);
        first_vals.push(new_vec[0]);
        swap(&mut old_vec, &mut new_vec);
        n_diff -= 1;
    }
    let new_end = last_vals.iter().sum();
    let mut new_start = 0;
    for v in first_vals.iter().rev() {
        new_start = v - new_start;
    }
    return (new_start, new_end);
}

pub fn day9(contents: &String) {
    let mut histories = Vec::new();
    for line in contents.lines() {
        let history: Vec<_> = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        histories.push(history)
    }
    let mut sum1 = 0;
    let mut sum2 = 0;
    for history in &histories {
        let (new_start, new_end) = create_values(history);
        sum1 += new_end; // Part 1
        sum2 += new_start; // Part 2
    }
    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");
}