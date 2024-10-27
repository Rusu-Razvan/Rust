use std::{fs, io};

fn format_line(s: String) -> String {
    let mut result_line = String::new();

    let mut v = s.split_whitespace();

    if let Some(first) = v.next() {
        result_line.push_str(first);
        result_line.push_str(" => ");

        if let Some(second) = v.next() {
            result_line.push_str(second);
        }
    }

    result_line.push('\n');
    result_line
}

fn read_prot() -> Result<(), io::Error> {
    let s: String = fs::read_to_string("C:/Windows/System32/drivers/etc/protocol")?; //hosts was fully commented

    let mut result_line = String::new();
    let mut result = String::new();

    for line in s.lines() {
        if !(line.to_string().starts_with("#")) {
            result_line = format_line(line.to_string());
            result.push_str(result_line.as_str());
        }
    }

    print!("{}", result);

    Ok(())
}

fn main() {
    println!("\n<protocol name> => <assigned number>");

    if let Err(err) = read_prot() {
        println!("{}", err);
    }
}
