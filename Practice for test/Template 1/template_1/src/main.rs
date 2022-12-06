// To develop a program in Rust that/to .... 
// ...comment.

use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");

    println!("Next information required from the customer");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let ..a number :i32 = input2.trim().parse().expect("Not a valid number");

    println!("Other information required from the customer");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let ..a number :i32 = input2.trim().parse().expect("Not a valid number");

    // Comment

    if ... >= .. && ... >= ..
    {
        println!("execute statement {}, the incentive...",input1);
    
    // Comment

    } else if ... >= .. && ... >= ..
    {
        println!("execute statement {}, the incentive...",input1);
    
    // Comment

    } else 
    {
        println!("execute statement {}, the incentive...",input1);
    }

}