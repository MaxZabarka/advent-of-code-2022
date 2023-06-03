use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let lines = read_lines("input.txt")?;

    let (max_sum, _) = lines.map(|line| line.expect("Could not read line")).fold(
        (0, 0),
        |(max_sum, cur_sum), line| {
            if let Ok(parsed_line) = line.parse::<i32>() {
                let new_sum = cur_sum + parsed_line;
                (max(max_sum, new_sum), new_sum)
            } else {
                (max_sum, 0)
            }
        },
    );
    println!("{:?}", max_sum);
    Ok(())
}

fn read_lines(filename: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
