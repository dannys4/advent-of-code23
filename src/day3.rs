fn process_char(c:char)->i32 {
    if c.is_numeric() {
        return c.to_digit(10).unwrap() as i32;
    }
    if c == '.' {
        return -1;
    }
    if c == '*' {
        return -2;
    }
    return -3;
}

fn find_num(v: &Vec<i32>, col: usize)->i32 {
    let mut idx = col;
    while idx > 0 && *v.get(idx-1).unwrap() >= 0 {
        idx -= 1;
    }
    let mut num = 0;
    while idx < v.len() && *v.get(idx).unwrap() >= 0 {
        num = num*10 + v.get(idx).unwrap();
        idx += 1;
    }
    return num;
}

fn check_for_num(v: &Vec<i32>, col: usize)->(i32,i32,i32) {
    let (mut left, mut right) = (false, false);
    let here = *v.get(col).unwrap() >= 0;
    if col > 0 {
        left = *v.get(col-1).unwrap() >= 0;
    }
    if col < v.len()-1 {
        right |= *v.get(col+1).unwrap() >= 0;
    }
    let is_num = left | right | here;
    if !is_num {
        return (-1, -1, -1);
    }
    let (mut left_num, mut here_num, mut right_num) = (-1, -1, -1);
    if here {
        here_num = find_num(v, col);
    } else {
        if left {
            left_num = find_num(v, col-1);
        }
        if right {
            right_num = find_num(v, col+1);
        }
    }
    return (left_num, here_num, right_num);
}


pub fn day3(contents: &String) {
    let mut vec = Vec::new();
    for line in contents.lines() {
        if line.len() <= 0 {
            break;
        }
        let arr: Vec<i32> = line.chars().map(process_char).collect();
        vec.push(arr);
    }
    let mut sum: u32 = 0;
    for (j, v) in vec.iter().enumerate() {
        let mut i = 0;
        while i < v.len() {
            let mut curr_num: u32 = 0;
            let mut add_to_sum = false;
            while i < v.len() && *v.get(i).unwrap() >= 0 {
                curr_num = curr_num*10 + *v.get(i).unwrap() as u32;
                // Check if to add to sum
                if i > 0 {
                    add_to_sum |= *v.get(i-1).unwrap() < -1;
                    if j > 0 {
                        add_to_sum |= *vec.get(j-1).unwrap().get(i-1).unwrap() < -1;
                    }
                    if j < vec.len()-1 {
                        add_to_sum |= *vec.get(j+1).unwrap().get(i-1).unwrap() < -1;
                    }
                }
                if i < v.len() - 1 {
                    add_to_sum |= *v.get(i+1).unwrap() < -1;
                    if j > 0 {
                        add_to_sum |= *vec.get(j-1).unwrap().get(i+1).unwrap() < -1;
                    }
                    if j < vec.len()-1 {
                        add_to_sum |= *vec.get(j+1).unwrap().get(i+1).unwrap() < -1;
                    }
                }
                if j > 0 {
                    add_to_sum |= *vec.get(j-1).unwrap().get(i).unwrap() < -1;
                }
                if j < vec.len()-1 {
                    add_to_sum |= *vec.get(j+1).unwrap().get(i).unwrap() < -1;
                }
                i = i+1;
            }
            i = i+1;
            if add_to_sum {
                // println!("Add {curr_num}...");
                sum = sum + curr_num;
            }
        }
    }
    println!("Part 1: {sum}");
    let mut ratios = 0;
    for (j, v) in vec.iter().enumerate() {
        for i in 0..v.len() {
            if *v.get(i).unwrap() != -2 {
                continue;
            }
            // At this point v[j][i] is a '*'
            let mut nums = Vec::new();
            if j > 0 {
                let (upleft, upmid, upright) = check_for_num(vec.get(j-1).unwrap(), i);
                nums.extend([upleft, upmid, upright]);
            }
            let (left, here, right) = check_for_num(v, i);
            nums.extend([left, here, right]);
            if j < vec.len()-1 {
                let (downleft, downmid, downright) = check_for_num(vec.get(j+1).unwrap(), i);
                nums.extend([downleft, downmid, downright]);
            }
            let nonneg_nums: Vec<&i32> = nums.iter().filter(|x| **x>=0).collect();
            // Check if there's *exactly two* numbers in nums, and add the product if so
            if nonneg_nums.len() == 2 {
                let num1 = *nonneg_nums.get(0).unwrap();
                let num2 = *nonneg_nums.get(1).unwrap();
                ratios += num1*num2;
            }
        }
    }
    println!("Part 2: {ratios}");
}