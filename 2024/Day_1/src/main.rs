use std::fs;
use std::io::Read;

fn main() {
    let mut file = fs::File::open("input.txt").unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read input");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in file_contents.lines() {
        let line_parts = line.split("  ").collect::<Vec<&str>>();

        left_list.push(line_parts[0].trim().parse::<i32>().unwrap());
        right_list.push(line_parts[1].trim().parse::<i32>().unwrap());
    }

    part_one(&mut left_list, &mut right_list);
    part_two(&mut left_list, &mut right_list);
}

fn part_one(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>)
{
    left_list.sort();
    right_list.sort();

    let mut distances: Vec<i32> = Vec::new();

    for i in 0..left_list.len() {
        distances.push((left_list[i] - right_list[i]).abs());
    }

    println!("Distances: {:?}", distances.iter().sum::<i32>())
}

fn part_two(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>)
{
    let mut similarity_score: i32 = 0;

    for number in left_list {
        similarity_score += *number * occurrences(*number, right_list)
    }

    println!("Similarity score: {}", similarity_score);
}

fn occurrences(number: i32, right_list: &Vec<i32>) -> i32 {
    let count = right_list.iter().filter(|&x| *x == number).count() as i32;

    count
}
