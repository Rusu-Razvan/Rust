fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }
    if number % 2 == 0 && number != 2 {
        return false;
    }

    let mut i: i32 = 3;

    while i * i <= number {
        if number % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn are_coprime(mut a: i32, mut b: i32) -> bool {
    let mut r: i32;
    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }

    if a == 1 {
        return true;
    } else {
        return false;
    }
}

fn bottles_of_beer() {
    let mut beers: i32 = 99;

    while beers > 1 {
        println!("{} bottles of beer on the wall,", beers);
        println!("{} bottles of beer.", beers);
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall.", beers - 1);

        println!(" ");
        beers -= 1;
    }

    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer.");
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
}

fn main() {
    let mut i:i32 = 0;

    while i<=100{
     if is_prime(i){
         print!("{} ",i);
     }
     i+=1;
    }

    println!();

    let a: i32=5;
    let b: i32=100;

    if are_coprime(a, b)
    {
     println!("{} and {} are coprime", a,b);
    }
    else {
     println!("{} and {} are not coprime",a,b);
    }
    
    bottles_of_beer();
}
