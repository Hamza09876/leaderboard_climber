use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'climbingLeaderboard' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY ranked
 *  2. INTEGER_ARRAY player
 */

fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut positions:Vec<i32> = vec![];
    let mut unique_ranked = vec![ranked[0]];

    // Remove duplicates from the ranked scores
    for i in 1..ranked.len() {
        if ranked[i] != ranked[i-1] {
            unique_ranked.push(ranked[i]);
        }
    }

    // Traverse the unique ranked scores in reverse order
    let mut i = unique_ranked.len() - 1;
    for score in player {
        while i > 0 && *score > unique_ranked[i] {
            i -= 1;
        }
        if *score >= unique_ranked[i] {
            positions.push(i + 1);
        } else {
            positions.push(i + 2);
        }
    }

    positions
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ranked_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ranked: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let _player_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let player: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = climbingLeaderboard(&ranked, &player);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
