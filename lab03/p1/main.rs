use std::u16;

fn checked_multiplication(n1: u16, n2: u16) -> Option<u16> {
    if n1 == 0 {
        Some(0 as u16)
    } else if u16::MAX / n1 < n2 {
        None
    } else {
        Some(n1 * n2)
    }
}

fn is_prime(n: u16) -> bool {
    if n <= 1 {
        return false;
    }
    if n != 2 && n % 2 == 0 {
        return false;
    }
    let mut i: u16 = 3;
    while checked_multiplication(i, i).expect("Overflow!") <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}

fn next_prime(n: u16) -> Option<u16> {
    if n >= u16::MAX - 1 {
        return None;
    }
    let mut i: u16 = n + 1;

    while i <= u16::MAX {
        if is_prime(i) {
            return Some(i);
        }
        i += 1;
    }

    return None;
}

fn main() {
    
    let mut x = next_prime(0);
    print!("{}", x.unwrap());

    while x.is_some() {
        x = next_prime(x.unwrap());
        
        match x {
            Some(value) => println!("{value}"),
            None => println!("Number too big!"),
        }
    }
}
