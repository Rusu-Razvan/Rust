enum MyErr {
    AdditionOverflow,
    MultiplicationOverflow,
}

fn checked_addition(n1: u32, n2: u32) -> Result<u32, MyErr> {
    if u32::MAX - n1 < n2 {
        Err(MyErr::AdditionOverflow)
    } else {
        Ok(n1 + n2)
    }
}

fn checked_multiplication(n1: u32, n2: u32) -> Result<u32, MyErr> {
    if n1 == 0 {
        Ok(0)
    } else if u32::MAX / n1 < n2 {
        Err(MyErr::MultiplicationOverflow)
    } else {
        Ok(n1 * n2)
    }
}

fn main() {
    let m = u32::MAX;
    let sum = checked_addition(23, 42);
    let product = checked_multiplication(2, m);

    if sum.is_err() {
        println!("Sum overflow!");
    } else {
        println!("The sum is {}", sum.ok().unwrap());
    }

    if product.is_err() {
        println!("Product overflow!");
    } else {
        println!("The product is {}", product.ok().unwrap());
    }
}
