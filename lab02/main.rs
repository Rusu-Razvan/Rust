/*fn add_chars_n(mut s: String, ch:char, number:i32) -> String
{
    let mut i:i32 = 0;
    while i < number {
        s.push(ch);
        i += 1;
    }
    s
}

fn add_chars_n_ref(s:&mut String , ch:char, number:i32)
{
    let mut i:i32 = 0;
    while i< number {
        s.push(ch);
        i += 1;
    }
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
         add_chars_n_ref(&mut s, c, 26 - i);
        //s = add_chars_n(s, c, 26 - i);
        i += 1;
    }

    print!("{}", s);
}*/


fn add_space(s:&mut String, number_of_spaces:i32)
{
    let mut i:i32 = 0;

    while i < number_of_spaces
    {
        s.push(' ');
        i += 1;
    }
}
                    


fn add_str(s:&mut String, s2:&str)
{
    s.push_str(s2);
}

fn total_digits(mut number:i32) -> i32
{
    let mut digits = 0;

    digits += 1;
    number /= 10;

    while number != 0
    {
        digits += 1;
        number /= 10;
    }

    digits
}

fn add_integer(s:&mut String, mut number:i32)
{
    let number_of_digits:i32 = total_digits(number);
    
    let mut digits_added_count = 0;
    
    let mut digit:u8;
    
    let mut p = 1;

    let mut i = 0;
    while i < number_of_digits - 1
    {
        p *= 10;
        i += 1;
    }

    while number != 0
    {
        
        digit = (number / p) as u8 + b'0';
        
        if digits_added_count == 3
        {
            s.push('_');
            digits_added_count = 0;
        }
        
        s.push(digit as char);
        
        digits_added_count += 1;
        number %= p;
        p=p/10;
    }
}


fn main()
{
  let mut s:String = String::from("bruh");
  add_str(&mut s, "wow");
  add_space(&mut s, 5);
  add_integer(&mut s, 3456789);
  print!("{} ",s);
}


