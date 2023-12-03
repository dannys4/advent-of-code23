pub fn day1(contents: &String) {
    let floor_dirs = contents.chars();
    let mut floor = 0;
    let mut found_basement = 0;
    for (j, c) in floor_dirs.enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor < 0 && found_basement == 0 {
            found_basement = j+1;
        }
    }
    println!("Ended on floor {floor}, first basement visit at step {found_basement}");
}