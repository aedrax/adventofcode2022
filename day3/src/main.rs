use std::fs::File;
use std::io::{BufRead, BufReader};


fn main_part1() {
    // open the file input.txt and read it line by line
    let file = File::open("input3.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    
    // create a running total
    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        // get the length of the line
        let length = line.len();
        // split the line at the index of half the length
        let (first, second) = line.split_at(length / 2);

        // create a mutable character
        let mut duplicate_item = ' ';
        // for each character in first, compare it to each character in second and if they are the same, set duplicate_item to that character
        for c in first.chars() {
            for d in second.chars() {
                if c == d {
                    duplicate_item = c;
                }
            }
        }

        // if the duplicate_item is still a space, continue
        if duplicate_item == ' ' {
            continue;
        }

        // add to the total 1 to 26 if the duplicate_item is a-z and 27 to 52 if the character is A-Z
        if duplicate_item.is_lowercase() {
            total += duplicate_item as u32 - 'a' as u32 + 1;
        } else {
            total += duplicate_item as u32 - 'A' as u32 + 27;
        }
        
    }
    println!("{}", total);
}


fn main() {
    // open the file input.txt and read it line by line
    let file = File::open("input3.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    
    // create a running total
    let mut total = 0;

    // for every three lines, create a vector of strings
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut duplicate_item = ' ';
        lines.push(line);
        if lines.len() == 3 {
            // find the character common in all three lines
            'charchecker: for c in lines[0].chars() {
                for d in lines[1].chars() {
                    for e in lines[2].chars() {
                        if c == d && d == e {
                            duplicate_item = c;
                            break 'charchecker;
                        }
                    }
                }
            }
            // clear the vector
            lines.clear();
            // if the duplicate_item is still a space, continue
            if duplicate_item == ' ' {
                continue;
            }
            
            // add to the total 1 to 26 if the duplicate_item is a-z and 27 to 52 if the character is A-Z
            if duplicate_item.is_lowercase() {
                total += duplicate_item as u32 - 'a' as u32 + 1;
            } else {
                total += duplicate_item as u32 - 'A' as u32 + 27;
            }
        }
    }
    println!("{}", total);
}