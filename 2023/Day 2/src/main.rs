use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, Read};

struct RequiredCubes {
    red: i32,
    green: i32,
    blue: i32
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    let sum_of_possible_game_ids= buffer
        .lines()
        .filter(
            |game| game_is_possible(game)
        )
        .map(|possible_game| extract_id(possible_game))
        .fold(0, |carry, id| carry + id);
    ;

    let power_of_games = buffer
        .lines()
        .map(|game| minimum_cubes_required(game))
        .fold(0, |carry, cubes| carry + (cubes.blue * cubes.red * cubes.green))
    ;

    println!("Part 1: {}", sum_of_possible_game_ids);
    println!("Part 2: {}", power_of_games);
}

fn game_is_possible(game: &str) -> bool {
    let max_blues = 14;
    let max_reds = 12;
    let max_greens = 13;

    let sets = game.split(":").collect::<Vec<&str>>()[1];

    for set in sets.split(";") {
        for cubes in set.split(",") {
            let tokens = cubes.trim().split(" ").collect::<Vec<&str>>();

            let amount   = tokens[0].parse::<i32>().unwrap();
            let category = tokens[1];

            if category == "blue" {
                if amount > max_blues {
                    return false;
                }
            }

            if category == "green" {
                if amount > max_greens {
                    return false;
                }
            }

            if category == "red" {
                if amount > max_reds {
                    return false;
                }
            }
        }

    }

    true
}

fn extract_id(game: &str) -> i32 {
    let game_components = game.split(":").collect::<Vec<&str>>();

    game_components[0].replace("Game ", "").parse::<i32>().unwrap()
}

fn minimum_cubes_required(game: &str) -> RequiredCubes {
    let sets = game.split(":").collect::<Vec<&str>>()[1];

    let mut highest_blues = 0;
    let mut highest_reds = 0;
    let mut highest_greens = 0;

    for set in sets.split(";") {
        for cubes in set.split(",") {
            let tokens = cubes.trim().split(" ").collect::<Vec<&str>>();

            let amount   = tokens[0].parse::<i32>().unwrap();
            let category = tokens[1];

            if category == "blue" {
                if amount > highest_blues {
                    highest_blues = amount;
                }
            }

            if category == "green" {
                if amount > highest_greens{
                    highest_greens = amount;
                }
            }

            if category == "red" {
                if amount > highest_reds {
                    highest_reds = amount;
                }
            }
        }
    }

    RequiredCubes{red: highest_reds, green: highest_greens, blue: highest_blues}
}