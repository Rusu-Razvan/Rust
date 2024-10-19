fn checked_addition(n1: u32, n2: u32) -> u32 {
    if u32::MAX - n1 < n2 {
        panic!("The addition overflows!");
    } else {
        n1 + n2
    }
}

fn checked_multiplication(n1: u32, n2: u32) -> u32 {
    if n1 == 0 {
        0
    } else if u32::MAX / n1 < n2 {
        panic!("The multiplication overflows!");
    } else {
        n1 * n2
    }
}

fn main() {
    let m = u32::MAX;
    let sum = checked_addition(23, 42);
    let product = checked_multiplication(2, m);
    println!("The sum is:{}", sum);
    println!("The product is:{}", product);
}
