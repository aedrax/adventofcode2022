use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main_part1 () {
    let file = File::open("input2.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    // create running total
    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");
        
        let opponent = parts.next().unwrap();
        let my_move = parts.next().unwrap();
        // add 1 to the total if my_move is X, 2 if my_move is Y, 3 if my_move is Z
        match my_move {
            "X" => total += 1,
            "Y" => total += 2,
            "Z" => total += 3,
            _ => (),
        }

        // if opponent is A and my_move is X, add 3 to the total
        // if opponent is B and my_move is Y, add 3 to the total
        // if opponent is C and my_move is Z, add 3 to the total
        // if opponent is A and my_move is Y, add 6 to the total
        // if opponent is B and my_move is Z, add 6 to the total
        // if opponent is C and my_move is X, add 6 to the total
        match (opponent, my_move) {
            ("A", "X") => total += 3,
            ("B", "Y") => total += 3,
            ("C", "Z") => total += 3,
            ("A", "Y") => total += 6,
            ("B", "Z") => total += 6,
            ("C", "X") => total += 6,
            _ => (),
        }
    }
    println!("{}", total);
}

fn main () {
    let file = File::open("input2.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    // create running total
    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");
        
        let opponent = parts.next().unwrap();
        let my_move = parts.next().unwrap();
        // add 1 to the total if my_move is X, 2 if my_move is Y, 3 if my_move is Z
        match my_move {
            "X" => total += 0,
            "Y" => total += 3,
            "Z" => total += 6,
            _ => (),
        }

        match (opponent, my_move) {
            ("A", "X") => total += 3,
            ("A", "Y") => total += 1,
            ("A", "Z") => total += 2,
            ("B", "X") => total += 1,
            ("B", "Y") => total += 2,
            ("B", "Z") => total += 3,
            ("C", "X") => total += 2,
            ("C", "Y") => total += 3,
            ("C", "Z") => total += 1,
            _ => (),
        }
    }
    println!("{}", total);
}