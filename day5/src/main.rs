fn main_part1() {
    // open input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // create an array of 9 vectors of characters that are empty
    let mut columns: Vec<Vec<char>> = vec![vec![]; 9];

    // create a boolean called still_parsing
    let mut still_parsing = true;

    // create a for loop for each line of the input
    for line in input.lines() {
        // if the line is empty continue
        if line.is_empty() {
            continue;
        }

        if still_parsing {
            // if the character at index 1 is == 1, break the loop
            if line.chars().nth(1).unwrap() == '1' {
                still_parsing = false;
                continue;
            }

            // for each column 0-9
            for i in 0..9 {
                // if the character at i*4+1 is not a space, add it to the vector at index i
                if line.chars().nth(i * 4 + 1).unwrap() != ' ' {
                    // get the character at i*4+1 and add it to the vector at the beginning
                    columns[i].insert(0, line.chars().nth(i * 4 + 1).unwrap());
                }
            }
            continue;
        }

        // split the line by spaces
        let split = line.split(' ').collect::<Vec<&str>>();

        // split[1] is an integer, create a for loop that iterates that many times
        for _ in 0..split[1].parse::<usize>().unwrap() {
            // split[3] is an integer, pop the vector at that index - 1 and store it in a variable
            let popped = columns[split[3].parse::<usize>().unwrap() - 1]
                .pop()
                .unwrap();
            // split[5] is an integer, push the popped variable to the vector at that index - 1
            columns[split[5].parse::<usize>().unwrap() - 1].push(popped);
        }
    }

    // print all the last elements of the vectors
    for i in 0..9 {
        println!("{}", columns[i].last().unwrap());
    }
}

fn main() {
    // open input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // create an array of 9 vectors of characters that are empty
    let mut columns: Vec<Vec<char>> = vec![vec![]; 9];

    // create a boolean called still_parsing
    let mut still_parsing = true;

    // create a for loop for each line of the input
    for line in input.lines() {
        // if the line is empty continue
        if line.is_empty() {
            continue;
        }

        if still_parsing {
            // if the character at index 1 is == 1, break the loop
            if line.chars().nth(1).unwrap() == '1' {
                still_parsing = false;
                continue;
            }

            // for each column 0-9
            for i in 0..9 {
                // if the character at i*4+1 is not a space, add it to the vector at index i
                if line.chars().nth(i * 4 + 1).unwrap() != ' ' {
                    // get the character at i*4+1 and add it to the vector at the beginning
                    columns[i].insert(0, line.chars().nth(i * 4 + 1).unwrap());
                }
            }
            continue;
        }

        // split the line by spaces
        let split = line.split(' ').collect::<Vec<&str>>();

        // split[1] is an integer, create a for loop that iterates that many times
        let crates_to_move = split[1].parse::<usize>().unwrap();

        let from_column = split[3].parse::<usize>().unwrap() - 1;
        let to_column = split[5].parse::<usize>().unwrap() - 1;

        // move a slice of crates from one column to another
        let split_index = columns[from_column].len() - crates_to_move;
        let crates = columns[from_column].split_off(split_index);
        columns[to_column].extend(crates);
    }

    // print all the last elements of the vectors
    for i in 0..9 {
        println!("{}", columns[i].last().unwrap());
    }
}