/*
The problem specification can be found here: https://adventofcode.com/2022/day/1
*/
fn main() {
    // load the calories file
    let input = include_str!("calories.txt");

    let mut max = 0; // tracks the max calories found for an elf
    let mut memo: u32 = 0; // used to aggregate calories for an elf

    // iterate the lines in the file, aggregating until an empty line is
    // encountered, at which point we compare memo against max
    for line in input.lines() {
        let total = match line {
            // ignore empty lines
            "" => Some(memo),
            // attempt to parse the str and add it's value to memo,
            // ignoring malformed strings
            str => {
                match str.parse::<u32>() {
                    Ok(calories) => memo += calories,
                    Err(e) => println!("ignoring malformed line, error = {}", e),
                };
                None
            }
        };

        // total will only have a value when we have reached the end
        // of an elf's entries
        match total {
            None => {} // do nothing
            Some(count) => {
                // compare the latest total against max, replacing it if greater
                if count > max {
                    max = count
                }
                memo = 0;
            }
        }
    }

    // output the max
    println!("max = {}", max);
}
