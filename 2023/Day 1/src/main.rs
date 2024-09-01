use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    let sum = buffer.lines().fold(
        0,
        |carry, line| {
            carry + get_numbers(line)
        },
    );

    println!("{:?}", sum);
}

fn get_numbers(sample: &str) -> i32 {
    if sample.len() == 0 {
        return 0;
    }

    let mut first: char= "A".parse().unwrap();
    let mut last: char = "A".parse().unwrap();

    let normalized = normalize(sample);

    for character in normalized.chars() {
        if !character.is_numeric() {
            continue;
        }

        if !first.is_numeric() {
            first = character;
        }

        last = character;
    }

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

fn normalize(line: &str) -> String {
    line
        .replace("oneight", "18")
        .replace("eightwo", "82")
        .replace("twone", "21")
        .replace("one", "1ne")
        .replace("two", "2wo")
        .replace("three", "3hree")
        .replace("four", "4our")
        .replace("five", "5ive")
        .replace("six", "6ix")
        .replace("seven", "7even")
        .replace("eight", "8ight")
        .replace("nine", "9ine")
}