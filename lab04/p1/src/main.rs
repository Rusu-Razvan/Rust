use std::{fs, io};

fn print_longest_lines() -> Result<(), io::Error> {
    let s: String = fs::read_to_string("src/text.txt")?;

    let mut longest_line: &str = "";
    let mut max: usize = 0;

    for line in s.lines() {
        if line.len() > max {
            max = line.len();
            longest_line = line;
        }
    }

    println!("{}", &longest_line);

    max = 0;
    longest_line = "";

    for line in s.lines() {
        let mut total_ch = 0;
        for _ in line.chars() {
            total_ch += 1;
        }

        if total_ch > max {
            max = total_ch;
            longest_line = line;
        }
    }

    println!("{}", &longest_line);

    Ok(())
}

fn main() {
    if let Err(err) = print_longest_lines() {
        println!("{}", err);
    }
}
