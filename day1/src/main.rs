use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // open the file input.txt and read it line by line
    let file = File::open("input1.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    // create a vector of caloric values
    let mut caloric_values = Vec::new();

    // create an integer to hold the calorie total
    let mut total = 0;

    // for every line, convert the line to an integer and add it to the total
    for line in reader.lines() {
        let line = line.unwrap();
        // if the line is empty, skip it
        if line == "" {
            // add the total to the vector
            caloric_values.push(total);
            // reset the total
            total = 0;
            continue;
        }

        let caloric_value = line.parse::<i32>().unwrap();
        total += caloric_value;
    }

    // sort the vector
    caloric_values.sort();

    // print the highest
    println!("Highest: {}", caloric_values[caloric_values.len() - 1]);
    // print the total of the top 3 values
    println!(
        "Top 3: {}",
        caloric_values[caloric_values.len() - 1]
            + caloric_values[caloric_values.len() - 2]
            + caloric_values[caloric_values.len() - 3]
    );
}
