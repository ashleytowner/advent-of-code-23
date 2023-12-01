use std::env;
use std::fs::read_to_string;

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
            let val = match num {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => num,
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
            };
            match line.find(num) {
                Some(x) => {
                    if x <= first_index {
                        first_index = x;
                        first = Some(val);
                    }
                }
                None => {}
            }
            match line.rfind(num) {
                Some(x) => {
                    if x >= last_index {
                        last_index = x;
                        last = Some(val);
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
        print!("{} => {}\n", line, whole_num);
        let value = whole_num.parse::<i32>().unwrap();
        total += value;
    }
    print!("{}", total);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let challenge = &args[1];
        let filename = &args[2];
        match challenge.as_ref() {
            "1" => first_challenge(filename),
            _ => panic!("Invalid Challenge"),
        }
    } else {
        panic!("Incorrect Parameters")
    }
}
