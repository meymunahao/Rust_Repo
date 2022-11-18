// Finding the roots of a quadratic equation

use std::io;

fn main() {
    // Input the values of a,b, and c

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    // Finding the discriminant. It determines the number and nature of the root

    let discriminant:f32 = (b*b) - (4.0*a*c);

    //If discriminant is positive, then there are two distinct roots

    if discriminant > 0.0
    {
        let root1:f32 = (-b + discriminant.sqrt()) / 2.0*a;
        let root2:f32 = (-b - discriminant.sqrt()) / 2.0*a;
        println!("The equation has exactly two real roots which are {} and {}",root1,root2);
    
    // If discriminant is zero, then there is exactly one real root

    } else if discriminant == 0.0
    {
        let root3:f32 = (-b + discriminant.sqrt()) / 2.0*a;
        println!("The equation has exactly one real root which is {}",root3);
    
    // If discriminant is negative, then there are no real roots 
    
     } else if discriminant < 0.0
     {
        println!("The equation has no real roots.");
    }
}