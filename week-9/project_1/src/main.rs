use std::io::Write;

fn main() {

    let lager = "Lager: 33 Export, Desperados, Goldberg, Gulder, Heinken, Star\n";
    let stout = "Stout: Legend, Turbo King, Williams\n";
    let non_alcoholic = "Non-alcoholic: Maltina, Amstel Malta, Malta Gold, Fayrouz\n";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Nigeria Brewery Limited\n".as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}

 // Project Assignment

            let 7:[&str;5] = [consulting ];
    println!("array is {:?}",city_arr);
    println!("array size is :{}", city_arr.len());


    //Essence
    // EY HR is restructuring their services for the new strategic plan of the fiscal year.
    // However, the restructuring will affect some staff members in the company.
    // Strategy, Consulting, People and workforce, Transactions and corporate finance, Tax and 
    // Assurance, along with their corresponding services.

    // Rust programming develop a application that calls code functions
    // and that will create a file with the staff name 
    // and save the department and corresponding services into the file.

    // Strategy by EY-Parthenon: Strategy consulting, Corporate and growth strategy, Transaction strategy and execution, Restructuring and turnaround strategy, Industry strategy, Digital business building, Commercial strategy.
    // Consulting: Analytics consulting services, Customer experience, Cybersecurity, strategy, risk, compliance and resilience, Digital transformation, Risk consulting services, Supply chain and operations, Technology transformation.
    // People and workforce: Change management and experience, HR transformation, Integrated workforce mobility, Learning and development consulting, Recognition and reward advisory, Workforce analytics, People and workforce.
    // Transactions and corporate finance: Corporate finance, Divestments and carve-outs, Sustainability and ESG Services, M&A advisory, M&A integration, M&A technology and tools, M&A advanced analytics.
    // Tax: Tax planning, Tax function operations, Tax policy and controversy, Global trade, Tax accounting, Tax compliance, Transaction tax.
    // Assurance: Audit services, Climate change and sustainaibility services, Financial accounting advisory services, Forensic and integrity services, Private client audit experience, Accounting Link, Assurance. 
    
    // Name, Department, Qualification, Code.
    // Aigbona Juliet, Consulting, B.Sc., 7.
    // Ehis Ero, Strategy, M.Sc., 9.
    // Adamu Sagamu, Tax, B.Sc., 8.
    // Akpevwe Iloka, Assurance, HND, 7.
    // Maria Akinsola, Transactions and corporate finance, M.Sc., 9.
    // Gbenga Daniels, People and workforce, HND, 8.

    // (e.g. fn code_7) and that will create a file with the staff name e.g.

use std::io;

fn checker(){

    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <= '9'
    {
        println!("Character '{}' is a digit", ch);
    }
    else{
        println!("Character '{}' is not a digit", ch);
    }
}

fn main() {
    //  calling function
    println!("Welcome! This progress checks whether a character variable contains a digit or not");
    checker()
}

// another
    let city_arr:[&str;5] = ["Abuja","Porthacourt","Maiduguri","Kano","Lagos"];
    println!("array is {:?}",city_arr);
    println!("array size is :{}", city_arr.len());

    for index in 0..5 {
        println!("City index {} is located in :{}",index, city_arr[index]);



// another
fn main() {

    let mut count = 0;

    for num in 1..21 {
        if num > 10 {
            println!("m={}",num);
            continue;
        }
        count+=1;
    }
    println!("The count of values greater than 10 (between 1 and 20)");
    // outputs
}
