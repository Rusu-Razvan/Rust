use std::{fs, io};

fn replace_abbreviation(s: &str) -> String {
    match s {
        "ptr" => "pentru".to_string(),
        "pt" => "pentru".to_string(),
        "dl" => "domnul".to_string(),
        "dna" => "doamna".to_string(),
        _ => s.to_string(),
    }
}

fn replace_in_file() -> Result<(), io::Error> {
    let s: String = fs::read_to_string("src/phrase.txt")?;

    let mut result: String = String::new();

    for v in s.split(" ") {
        let replace_word = replace_abbreviation(v);
        result.push_str(&replace_word);
        result.push(' ');
    }

    fs::write("src/phrase.txt", &result)?;

    Ok(())
}

fn main() {
    if let Err(err) = replace_in_file() {
        println!("{}", err);
    }
}
