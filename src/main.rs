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
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        let mut got_first: bool = false;
        for c in line.chars() {
            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if !got_first {
                        first = Some(c);
                        got_first = true;
                    }
                    last = Some(c);
                }
                _ => {}
            }
        }
        let mut whole_num = match first {
            Some(x) => x.to_string().to_owned(),
            None => panic!()
        };
        match last {
            Some(x) => {
                whole_num.push_str(&x.to_string());
            },
            None => panic!()
        }
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
