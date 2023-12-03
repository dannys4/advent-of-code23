mod day1;
mod day2;
use std::fs;


fn setup(day: i32) -> String {
    let filename = format!("data/day{}.txt", day);
    let contents = fs::read_to_string(filename).expect("Could not read file");
    return contents;
}

fn main() {
    let days: Vec<&dyn Fn(&String) -> ()> = vec![&day1::day1, &day2::day2];
    let mut day = 1;
    for f in &days {
        println!("\nDay {day} result:\n");
        let contents_j = setup(day);
        (f)(&contents_j);
        day += 1;
    }
}
