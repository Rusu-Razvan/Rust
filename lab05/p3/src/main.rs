use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn find_oldest_and_youngest(students: &[Student]) -> Option<(Student, Student)> {
    let mut oldest = &students[0];
    let mut youngest = &students[0];

    for stud in students {
        if stud.age > oldest.age {
            oldest = stud;
        }
        if stud.age < youngest.age {
            youngest = stud;
        }
    }

    Some((oldest.clone(), youngest.clone()))
}

fn main() {
    let content = fs::read_to_string("students.json").unwrap();
    let students: [Student; 4] = serde_json::from_str(&content).unwrap();

    if let Some((oldest, youngest)) = find_oldest_and_youngest(&students) {
        println!("Oldest student: {:?}", oldest);
        println!("Youngest student: {:?}", youngest);
    } else {
        println!("No students found");
    }
}
