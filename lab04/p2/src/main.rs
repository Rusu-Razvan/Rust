fn rot13(s: &String) -> Result<String, &'static str> {
    let mut result: String = String::new();
    //let mut new_char:char=

    for c in s.chars() {
        if c.is_ascii() {
            let new_char: char = if (c >= 'A' && c <= 'M') || (c >= 'a' && c <= 'm') {
                (c as u8 + 13) as char
            } else if (c > 'M' && c <= 'Z') || (c > 'm' && c <= 'z') {
                (c as u8 - 13) as char
            } else {
                c
            };
            result.push(new_char);
        } else {
            return Err("Not ascii");
        }
    }
    Ok(result)
}

fn main() {
    let s = String::from("Jul qvq gur puvpxra pebff gur ebnq?\nGb trg gb gur bgure fvqr!");
    let mut encrypted_text: String = String::new();

    match rot13(&s) {
        Ok(result) => {
            println!("{}", result);
            encrypted_text.push_str(&result);
        }
        Err(err) => println!("{}", err),
    }

    println!();

    match rot13(&encrypted_text) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}", err),
    }
}
