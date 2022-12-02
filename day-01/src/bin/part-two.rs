use std::collections::BTreeSet;

/*
The problem specification can be found here: https://adventofcode.com/2022/day/1
*/
fn main() {
    // load the calories file
    let input = include_str!("../calories.txt");

    let mut memo: u32 = 0; // used to aggregate calories for an elf
    let mut set = BTreeSet::<u32>::new();

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
                set.insert(count);
                memo = 0;
            }
        }
    }

    let mut pop_count = 0;
    let mut top_three = 0;

    for entry in set.iter().rev() {
        top_three += entry;
        pop_count += 1;
        if pop_count == 3 {
            break;
        }
    }

    // output the max
    println!("top 3 = {}", top_three);
}
