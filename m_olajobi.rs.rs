// Rust program to merge seperate arrays into one single output.

use std::io;
fn geopo_merger() {


    println!("Welcome to The Geopolitical zoning division!");
    let name_arr:[&str; 5] = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu",
                           "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", 
                           "Osazuwa Faith Etieye"];
    println!("\nArray with");
    println!("array is {:?}",name_arr);
    println!("array size is :{}"name_arr.len());

    let ministry_arr:[&str; 5] = ["Internal Affairs", "Justice", "Defense",
                            "Power & Steel", "Petroleum"];
    println!("\nArray with");
    println!("array is {:?}", ministry_arr);
    println!("array size is :{}"ministry_arr.len());

    let geopoliticalzone_arr:[&str; 5] = ["South West", "North East", "South South",
                                        "South West", "South East"];
    println!("\nArray with");
    println!("array is {:?}", geopoliticalzone_arr);
    println!("array size is :{}"geopoliticalzone_arr.len());
    
}

// Rust program to check and validate staff employment position level for promotions.

use std::io;
fn pub_service() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Welcome to The Public Service Checker!");
    println!("Are you a Public Servant?");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");

    println!("What work do you do in the Public Service?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");

    println!("How many years have you worked for?");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let years :i32 = input3.trim().parse().expect("Not a valid number");
    
    For "Office Administrator" 

    if years >1 && years <= 2
    {
        println!("You are an Intern {}, ",input2);
  
   else if years >= 3 && years < 5
    {
        println!("You are an Administrator {}, ",input2);
  
   else if years >=5 && years < 8
    {
        println!("You are a Senior Administrator {}, ",input2);
  
   else if years >=8 && years < 10
    {
        println!("You are an Office Manager {}, ",input2);
  
   else if years >10 && years <= 13
    {
        println!("You are a Director {}, ",input2);
  
   else years > 14
    {
        println!("You are an CEO {}, ",input2);

For "Academic"
  
   else if years >= 3 && years < 5
    {
        println!("You are a Research Assistant {}, ",input2);
  
   else if years >=5 && years < 8
    {
        println!("You are a PhD Candidate {}, ",input2);
  
   else if years >=8 && years < 10
    {
        println!("You are a Post-Doc Researcher {}, ",input2);
  
   else if years >10 && years <= 13
    {
        println!("You are a Senior Lecturer {}, ",input2);
  
   else years > 14
    {
        println!("You are a Dean {}, ",input2);

    For "Lawyer"
  
    if years >1 && years <= 2
    {
        println!("You are a Paralegal {}, ",input2);
  
   else if years >= 3 && years < 5
    {
        println!("You are a Junior Associate {}, ",input2);
  
   else if years >=5 && years < 8
    {
        println!("You are an Associate {}, ",input2);
  
   else if years >=8 && years < 10
    {
        println!("You are a Senior Associate 1-2 {}, ",input2);
  
   else if years >10 && years <= 13
    {
        println!("You are a Senior Associate 3-4 {}, ",input2);
  
   else years > 14
    {
        println!("You are a Partner {}, ",input2);

 For "Teacher"
  
    if years >1 && years <= 2
    {
        println!("You are a Placement {}, ",input2);
  
   else if years >= 3 && years < 5
    {
        println!("You are a Classroom Teacher {}, ",input2);
  
   else if years >=5 && years < 8
    {
        println!("You are a Snr Teacher {}, ",input2);
  
   else if years >=8 && years < 10
    {
        println!("You are a Leading Teacher {}, ",input2);
  
   else if years >10 && years <= 13
    {
        println!("You are a Deputy Prinicipal {}, ",input2);
  
   else years > 14
    {
        println!("You are a Principal {}, ",input2);
}
