
#[derive(Copy, Clone)]
enum Color {RED=0, GREEN=1, BLUE=2, INVALID}

fn find_color(color: &str)->Color {
    if color == "red" {
        return Color::RED;
    }
    if color == "green" {
        return Color::GREEN;
    }
    if color == "blue" {
        return Color::BLUE;
    }
    return Color::INVALID;
}

fn read_game(game_set: &str)->(i32,i32,i32,i32)  {
    let (game_num_str, games) = game_set.rsplit_once(": ").unwrap();
    let game_num = (&game_num_str[5..]).parse::<i32>().unwrap();
    let mut ret = [0,0,0];
    for game in games.split("; ") {
        for obs in game.split(", ") {
            let (num_str, col) = obs.rsplit_once(' ').unwrap();
            let col_enum = find_color(col);
            let num = num_str.parse::<i32>().unwrap();
            if ret[col_enum as usize] < num {
                ret[col_enum as usize] = num;
            }
        }
    }
    let red = ret[Color::RED as usize];
    let green = ret[Color::GREEN as usize];
    let blue = ret[Color::BLUE as usize];
    return (game_num, red, green, blue);
}

pub fn day2(contents: &String) {
    let (find_red, find_green, find_blue) = (12, 13, 14);
    let mut game_sum = 0;
    for game_set in contents.lines() {
        if game_set.len() == 0 {
            break;
        }
        let (game_num, red, green, blue) = read_game(game_set);
        if red <= find_red && green <= find_green && blue <= find_blue {
            game_sum += game_num;
        }
    }
    println!("Part 1: {game_sum}");
    let mut set_power_sum = 0;
    for game_set in contents.lines() {
        if game_set.len() == 0 {
            break;
        }
        let (_, red, green, blue) = read_game(game_set);
        set_power_sum += red*green*blue;
    }
    println!("Part 2: {set_power_sum}");
}