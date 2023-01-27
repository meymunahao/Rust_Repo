use std::io::Write;

fn main() {

    let s = "student_name\n";
    let student_name = ["Student Name: Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Balanca Edemoh\n"];
    println!("array is {:?}", student_name);
    println!("array size is :{}", student_name.len());

    let m = "matric_number\n";
    let matric_number = ["Matric Number: ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001\n"];
    println!("array is {:?}", matric_number);
    println!("array size is :{}", matric_number.len());

    let d = "department\n";
    let department = ["Department: Accounting","Economics","Computer","Electrical","Mechanical\n"];
    println!("array is {:?}", department);
    println!("array size is :{}", department.len());

    let l = "level\n";
    let level = ["Level: 300","100","200","200","100\n"];
    println!("array is {:?}", level);
    println!("array size is :{}", level.len());

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to PAU SMIS\n"
        .as_bytes()).expect("write failed");
    file.write_all(s.as_bytes()).expect("write failed");
    file.write_all(m.as_bytes()).expect("write failed");
    file.write_all(d.as_bytes()).expect("write failed");
    file.write_all(l.as_bytes()).expect("write failed");
    println!("\nData written to file.");

}
