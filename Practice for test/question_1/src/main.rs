// Rust program to build a Faculty Publication incentive system 
// for only 50 faculty members
// name and number of papers published by the faculty member(only 50 of them), then...
// ...displays name and incentive that'll be obtained



use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");

    println!("Enter the numbers of papers you have published");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let paper :i32 = input2.trim().parse().expect("Not a valid number");

    // If faculty has published >= 3 <= 5 papers, then incentive is 500,000.

    if paper >= 3 && paper < 5
    {
        println!("Good, you're on your way to being a great publisher {}, your incentive is 500,000",input1);
    
    // If faculty has published >= 5 <= 10 , then incentive is 800,000.

    } else if paper >= 5 && paper < 10
    {
        println!("Nice, you've published quite a number {}, your incentive is 800,000",input1);
    
    // If faculty has published >= 10, then incentive is 1,000,000.

    } else if paper >= 10
    {
        println!("Great, you've published a lot {}, your incentive is 1,000,000",input1);
    
    // For faculty < 3, icentive is 100,000.

    } else 
    {
        println!("You haven't published enough yet {}, your incentive is 100,000",input1);
    }

}