// A Rust program that displays a menu for food items available to take order from the customer.
// The program inputs the type of food and quantity.
// It finally displays the total charges for the order according to price criteria. 

use std::io;

fn main() {

    let p = "1        Poundo Yam/Edinkaiko Soup     -N3,200".to_string();
    let f = "2        Fried Rice & Chicken          -N3,000".to_string();
    let a = "3        Amala & Ewedu Soup            -N2,500".to_string();
    let e = "4        Eba & Egusi                   -N2,000".to_string();
    let w = "5        White Rice & Stew             -N2,000".to_string();

    println!("Welcome! Menu for food items available");

    println!("\nCode     Menu                           Prices");
    let menu = format!("{} \n{} \n{} \n{} \n{}",p,f,a,e,w);

    println!("{}",menu);

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nPlease enter the code for what you would like to eat today.");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let food:i64 = input1.trim().parse().expect("Not a valid number");
    

    println!("How many packs would you like to buy?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f64 = input2.trim().parse().expect("Not a valid number");

    let mut t:f64 = 0.0;  // t is the total amount.
    let z:f64;     // z is the new amount after the discount has been calculated.
    let m:f64 = 10000.0;

    if food == 1 {
        t = quantity * 3200.0;
        println!("Your order is Poundo Yam/Edinkaiko Soup and the total amount is N{}.",t);

    } else if food == 2 {
        t = quantity * 3000.0;
        println!("Your order is Fried Rice & Chicken and the total amount is N{}.",t);

    } else if food == 3 {
        t = quantity * 2500.0;
        println!("Your order is Amala & Ewedu Soup and the total amount is N{}.",t);

    } else if food == 4 {
        t = quantity * 2000.0;
        println!("Your order is Eba & Egusi and the total amount is N{}.",t);

    } else if food == 5 {
        t = quantity * 2000.0;
        println!("Your order is White Rice & Stew and the total amount is N{}.",t);

    } else {
        println!("Please type in a code from the menu.");
    }

    // If the total order is greater than N10,000, give a  discount of 5%.

    if t > m {
        z =  t - ((5.0/100.0) * t);
        println!("Congratulations! You have a 5% discount. The new amount is N{}.",m);

    }
}