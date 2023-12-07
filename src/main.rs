mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
use std::fs;


fn setup(day: i32) -> String {
    let filename = format!("data/day{}.txt", day);
    let contents = fs::read_to_string(filename).expect("Could not read file");
    return contents;
}

fn main() {
    let days: Vec<(&dyn Fn(&String) -> (), bool)> = vec![
        (&day1::day1, true), (&day2::day2, true),
        (&day3::day3, true), (&day4::day4, true),
        (&day5::day5, false), // Takes awhile
        (&day6::day6, true)];
    let mut day = 1;
    for (f, show) in &days {
        if *show {
            println!("\nDay {day} result:\n");
            let contents_j = setup(day);
            (f)(&contents_j);
        }
        day += 1;
    }
}
