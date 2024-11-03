use std::{fs, io};

#[derive(Debug, Clone)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn string_to_u32(s: &str) -> Option<u32> {
    let mut n = 0;

    for c in s.chars() {
        if let Some(digit) = c.to_digit(10) {
            n = n * 10 + digit;
        } else {
            return None;
        }
    }
    return Some(n);
}

fn parse_students(line: &str) -> Option<Student> {
    let mut count = 0;
    let mut name = String::new();
    let mut phone = String::new();
    let mut age: u32 = 0;

    for info in line.split(',') {
        match count {
            0 => name = info.to_string(),
            1 => phone = info.to_string(),
            2 => {
                if let Some(parsed_age) = string_to_u32(info.trim()) {
                    age = parsed_age;
                } else {
                    return None;
                }
            }
            _ => break,
        }
        count += 1;
    }

    if count == 3 {
        Some(Student { name, phone, age })
    } else {
        None
    }
}

fn find_oldest_and_youngest(students: &[Option<Student>]) -> Option<(Student, Student)> {
    let mut oldest: Option<Student> = None;
    let mut youngest: Option<Student> = None;

    let mut oldest_age = 0;
    let mut youngest_age = u32::MAX;

    for i in students {
        if let Some(student) = i {
            if oldest.is_none() || student.age > oldest_age {
                oldest = Some(student.clone());
                oldest_age = student.age;
            }

            if youngest.is_none() || student.age < youngest_age {
                youngest = Some(student.clone());
                youngest_age = student.age;
            }
        }
    }

    if let (Some(oldest), Some(youngest)) = (oldest, youngest) {
        Some((oldest, youngest))
    } else {
        None
    }
}

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("info.txt")?;

    let mut students: [Option<Student>; 4] = [None, None, None, None];

    let mut i = 0;

    for line in input.lines() {
        if i >= students.len() {
            break;
        }
        if let Some(student) = parse_students(line) {
            students[i] = Some(student);
        }
        i += 1;
    }

    if let Some((oldest, youngest)) = find_oldest_and_youngest(&students) {
        println!("The oldest is: {:?}", oldest);
        println!("The youngest is: {:?}", youngest);
    } else {
        println!("No students found");
    }

    Ok(())
}
