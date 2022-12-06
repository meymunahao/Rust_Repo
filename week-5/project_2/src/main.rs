// To develop a program in Rust that takes as input 
// the experience and age of an employee 
// to determine the annual incentive.

use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");

    println!("How long have you worked with us?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let years :i32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let age:i32 = input3.trim().parse().expect("Not a valid number");

    // If the employee is experienced and his/her age 
    // is => 40, the incentive is N1,560,000. 

    if years >= 10 && age >= 40 
    {
        println!("Great, you're experienced {}, your incentive is N1,560,000",input1);
    
    // If the employee is experienced and his/her age 
    // is => 30 but < 40, then incentive is N1,480,000. 


    } else if years >= 10 && age >= 30
    {
        println!("Great, you're experienced {}, your incentive is N1,480,000",input1);
    
    // For experienced employee < 28 years of age 
    // the incentive is N1,300,000.


    } else if years >= 10 && age < 28
    {
        println!("Great, you're experienced {}, your incentive is N1,300,000",input1);
   
   // For inexperienced employee the incentive is  N100,000.


    } else 
    {
        println!("You are not experienced enough {}, your incentive is N100,000",input1);
    }

}