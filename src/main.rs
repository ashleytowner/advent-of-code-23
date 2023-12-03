use std::env;
use std::fs::read_to_string;

#[macro_export]
macro_rules! word_to_num {
    ( $x:expr ) => {
        match $x {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => $x,
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => "",
        }
    };
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn first_challenge(filename: &str) {
    let lines = read_lines(filename);
    let mut total = 0;
    for line in lines {
        let mut first: Option<&str> = None;
        let mut first_index = usize::MAX;
        let mut last: Option<&str> = None;
        let mut last_index: usize = 0;
        let nums = vec![
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four",
            "five", "six", "seven", "eight", "nine",
        ];

        for num in nums {
            match line.find(num) {
                Some(x) => {
                    if x <= first_index {
                        first_index = x;
                        first = Some(word_to_num!(num));
                    }
                }
                None => {}
            }
            match line.rfind(num) {
                Some(x) => {
                    if x >= last_index {
                        last_index = x;
                        last = Some(word_to_num!(num));
                    }
                }
                None => {}
            }
        }

        let mut whole_num = match first {
            Some(x) => x.to_string().to_owned(),
            None => panic!("No First Num"),
        };
        match last {
            Some(x) => {
                whole_num.push_str(&x.to_string());
            }
            None => panic!("No Last Num"),
        }
        let value = whole_num.parse::<i32>().unwrap();
        total += value;
    }
    print!("{}", total);
}

fn second_challenge(filename: &str) {
    let lines = read_lines(filename);
    let mut valid_game_sum = 0;
    let mut power_sum = 0;
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    for line in lines {
        let parts = line.split(":").collect::<Vec<&str>>();
        let game_id = parts[0].split(" ").collect::<Vec<&str>>()[1];
        let draws = parts[1].split(";");
        let mut red_high = 0;
        let mut green_high = 0;
        let mut blue_high = 0;
        for draw in draws {
            let colours = draw.split(", ");
            for colour in colours {
                let parts = colour.trim().split(" ").collect::<Vec<&str>>();
                let count = parts[0].parse::<i32>().unwrap();
                match parts[1] {
                    "red" => {
                        if count > red_high {
                            red_high = count;
                        }
                    }
                    "green" => {
                        if count > green_high {
                            green_high = count;
                        }
                    }
                    "blue" => {
                        if count > blue_high {
                            blue_high = count;
                        }
                    }
                    _ => {}
                }
            }
        }
        if (red_high <= red_max) && (green_high <= green_max) && (blue_high <= blue_max) {
            valid_game_sum += game_id.parse::<i32>().unwrap();
        }
        let power = red_high * green_high * blue_high;
        power_sum += power;
    }
    print!("{}\n{}\n", valid_game_sum, power_sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let challenge = &args[1];
        let filename = &args[2];
        match challenge.as_ref() {
            "1" => first_challenge(filename),
            "2" => second_challenge(filename),
            _ => panic!("Invalid Challenge"),
        }
    } else {
        panic!("Incorrect Parameters")
    }
}
