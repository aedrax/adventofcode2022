fn main_part1() {
    // open input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // read the input until there are 4 unique characters in a row
    let mut marker = 4;
    
    // loop from marker to the end of the input
    for i in marker..input.len() {
        // add the characters from i -4 to i to a set
        let mut set = std::collections::HashSet::new();
        for j in i - 4..i {
            set.insert(input.chars().nth(j).unwrap());
        }

        // if the length of set is 4, set the marker to i and break
        if set.len() == 4 {
            //print set
            println!("{:?}", set);
            marker = i;
            break;
        }
    }

    // print the marker
    println!("{}", marker);

}

fn main() {
    // open input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // read the input until there are 4 unique characters in a row
    let mut marker = 14;
    
    // loop from marker to the end of the input
    for i in marker..input.len() {
        // add the characters from i - 14 to i to a set
        let mut set = std::collections::HashSet::new();
        for j in i - 14..i {
            set.insert(input.chars().nth(j).unwrap());
        }

        // if the length of set is 14, set the marker to i and break
        if set.len() == 14 {
            //print set
            println!("{:?}", set);
            marker = i;
            break;
        }
    }

    // print the marker
    println!("{}", marker);

}
