// import all needed
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // open input4.txt
    let file = File::open("input4.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    // create running total
    let mut total_contained = 0;
    let mut total_pairs = 0;

    // read each line of the file
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(",");

        let elf_a = parts.next().unwrap();
        let elf_b = parts.next().unwrap();

        // split elf_a into two ints separated by -
        let mut elf_a_parts = elf_a.split("-");
        let elf_a_start = elf_a_parts.next().unwrap().parse::<i32>().unwrap();
        let elf_a_end = elf_a_parts.next().unwrap().parse::<i32>().unwrap();

        // split elf_b into two ints separated by -
        let mut elf_b_parts = elf_b.split("-");
        let elf_b_start = elf_b_parts.next().unwrap().parse::<i32>().unwrap();
        let elf_b_end = elf_b_parts.next().unwrap().parse::<i32>().unwrap();

        if (elf_a_start >= elf_b_start && elf_a_end <= elf_b_end)
            || (elf_b_start >= elf_a_start && elf_b_end <= elf_a_end)
        {
            total_contained += 1;
        }

        if ((elf_a_start >= elf_b_start && elf_a_start <= elf_b_end) ||
            (elf_a_end >= elf_b_start && elf_a_end <= elf_b_end) ||
            (elf_b_start >= elf_a_start && elf_b_start <= elf_a_end) ||
            (elf_b_end >= elf_a_start && elf_b_end <= elf_a_end) ||
            (elf_a_start <= elf_b_start && elf_a_end >= elf_b_end) ||
            (elf_b_start <= elf_a_start && elf_b_end >= elf_a_end))
        {
            total_pairs += 1;
        }
    }
    println!("Total overlapped pairs: {}", total_contained);
    println!("Total pairs: {}", total_pairs);
}
