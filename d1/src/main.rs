use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// 69528

fn main() {
    let mut max = 0;
    if let Ok(lines) = read_lines("./data.txt") {
        let mut calories = 0;
        for line in lines {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        if max < calories {
                            max = calories;
                        }

                        calories = 0;
                    } else {
                        calories += u32::from_str_radix(&line, 10).unwrap();
                    }
                }
                Err(_) => {
                    if max < calories {
                        max = calories;
                    }

                    println!("{}", max);
                }
            }
        }
        println!("{}", max);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
