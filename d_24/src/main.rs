use std::io;
#[derive(Copy, Clone)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Left = 2,
    Right = 4,
    Up = 8,
    Down = 16,
}

struct Grid {
    grid: Vec<Vec<u8>>,
}

fn main() -> Result<(), io::Error> {
    println!("{:?}", std::env::current_dir());
    let input = parse_input("./input2.txt")?;
    // input.iter().fold(0, |sum ,acc| {
    //     sum+acc
    // });
    let (grid, _, _) = input.iter().fold(
        (vec![vec![]], 0, 0),
        |(mut grid, row, col): (Vec<Vec<u8>>, usize, usize), c: &u8| {
            print!("{:?}", *c as char);
            if *c == b'\r' {
                return (grid, row, col);
            }
            if *c == b'\n' {
                return (grid, 0, col + 1);
            }
            grid[col].push(*c);
            (grid, row + 1, col)
        },
    );
    println!("{:?}", grid);
    Ok(())
}

fn parse_input(filename: &str) -> Result<Vec<u8>, io::Error> {
    Ok(std::fs::read(filename)?)
}
