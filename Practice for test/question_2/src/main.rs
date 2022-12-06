// Rust program to build a student council voter system for only 15 eligible candidates.
// The function checks if the candidate is a current class rep., 
// not in 100 level and has a CGPA above  4.0.
// If the candidate meets all requirements, the function displays
// name, email, department and state of origin, 
// with a message "You can vote",
// else "Sorry, you are not eligible to vote".

use std::io;

fn facbub(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    let yes:bool = true;


    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");

    println!("Are you currently a class rep? {}", yes);
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let rep :bool = input2.trim().parse().expect("Not a valid input");

    println!("Are you in 200, 300, 400, or 500 level? ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let level :bool = input3.trim().parse().expect("Not a valid input");

    println!("Do you have a cgpa above  4.0?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let cgpa :bool = input4.trim().parse().expect("Not a valid input")

    // If all requirements are met..

    if rep 
    {
        println!(" You are eligible to vote {}",input1);
    
    // If all the requirements are not met, then...

    } else if no 
    {
        println!("Sorry, you're not eligible to vote {}",input1);
}
        fn main(){
            println!("Faculty publication System");
            facbub()
}