use std::io;
use std::io::Read;

fn administrator() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn project_manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn customer() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

 fn main(){
    println!("Welcome!");

    

    println!("If you are an administrator, type 1.");
    println!("If you are a project manager, type 2.");
    println!("If you are an employee, type 3.");
    println!("If you are a customer, type 4.");
    println!("If you are a vendor, type 5.");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let choice:i32 = input1.trim().parse().expect("Invalid input");
   
    if choice == 1
    {
        administrator();
    } else if choice == 2
    {
        project_manager();
    } else if choice == 3
    {
        employee();
    } else if choice == 4
    {
        customer();
    } else if choice == 5
    {
        vendor();
    } else
    {
        println!("Please select a number from 1 to 5.");
    }
 }
