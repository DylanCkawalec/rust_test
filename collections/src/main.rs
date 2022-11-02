fn main() {
    let mut students = vec![Student {
        name: "Ryan".to_string(),    
    }];

    students.push(Student { 
        name: "Lisa".to_string(),
    });

    assert!(&students[0]==&Student{name: "Ryan".to_string()});

    assert!(students.get(0) == Some(&Student { name: "Ryan".to_string()}));

    assert!(students.get(10000) == None);

    for student in students.iter(){
        println!("student name: {}", student.name);
    }

    use std::collections::HashMap; 

    let mut enrollment = Hashmap::new();
    enrollment.insert("biology".to_string(), students);

    let bio_students = enrollment.get("biology");
    let students = enrollment.remove("biology");


}

#[derive(PartialEq, Eq)]
struct Student { 
    name : String,
}