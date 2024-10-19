//#[derive(Debug)]

enum MyErrors {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, MyErrors> {
    if !((c > 'a' && c < 'z') || (c > 'A' && c < 'Z')) {
        return Err(MyErrors::NotLetter);
    }

    if c > 'a' && c < 'z' {
        Ok(((c as u8) - 32) as char)
    } else {
        Ok(c)
    }
}

fn to_lowercase(c: char) -> Result<char, MyErrors> {
    if !((c > 'a' && c < 'z') || (c > 'A' && c < 'Z')) {
        return Err(MyErrors::NotLetter);
    }

    if c >= 'A' && c <= 'Z' {
        Ok((c as u8 + 32) as char)
    } else {
        Ok(c)
    }
}

fn print_char(c: char) -> Result<(), MyErrors> {
    if c < ' ' || c > '~' {
        return Err(MyErrors::NotPrintable);
    }
    println!("{}", c);
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, MyErrors> {
    if c < '0' || c > '9' {
        if c > '\u{007F}' {
            return Err(MyErrors::NotAscii);
        }

        return Err(MyErrors::NotDigit);
    }
    Ok((c as u8 - b'0') as u32)
}

fn char_to_number_hex(c: char) -> Result<u32, MyErrors> {
    if c > '\u{007F}' {
        return Err(MyErrors::NotAscii);
    }

    if (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f') {
        if c >= '0' && c <= '9' {
            return Ok((c as u8 - b'0') as u32); // '0'-'9'
        }
        if c >= 'A' && c <= 'F' {
            return Ok((c as u8 - b'A' + 10) as u32); // 'A'-'F'
        }
        if c >= 'a' && c <= 'f' {
            return Ok((c as u8 - b'a' + 10) as u32); // 'a'-'f'
        }
    }
    Err(MyErrors::NotBase16)
}

fn print_error(error: MyErrors) {
    match error {
        MyErrors::NotAscii => println!("Error: Character is not ASCII."),
        MyErrors::NotDigit => println!("Error: Character is not a digit."),
        MyErrors::NotBase16 => println!("Error: Character is not a base16 digit."),
        MyErrors::NotLetter => println!("Error: Character is not a letter."),
        MyErrors::NotPrintable => println!("Error: Character is not printable."),
    }
}

fn main() {
    let c = 'E';
    match to_uppercase(c) {
        Ok(upper) => println!("Uppercase of '{}': {}", c, upper),
        Err(e) => print_error(e),
    }

    match to_lowercase(c) {
        Ok(lower) => println!("Lowercase of '{}': {}", c, lower),
        Err(e) => print_error(e),
    }

    match print_char(c) {
        Ok(()) => println!("Printed character: {}", c),
        Err(e) => print_error(e),
    }

    match char_to_number(c) {
        Ok(num) => println!("Numeric value of '{}': {}", c, num),
        Err(e) => print_error(e),
    }

    match char_to_number_hex(c) {
        Ok(num) => println!("Hexadecimal value of '{}': {}", c, num),
        Err(e) => print_error(e),
    }
}
