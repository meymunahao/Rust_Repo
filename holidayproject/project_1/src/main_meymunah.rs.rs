// Creating and Writing in the files. strategyplan_service
use std::io::Write;

    if code == 7{
        println!("");
        fn code_7() {

        let consulting = {"Services: \nAnalytics consulting services, \nCustomer experience, \nCybersecurity, strategy, risk, compliance and resilience, 
        Digital transformation, \nRisk consulting services, \nSupply chain and operations, \nTechnology transformation."};
        let mut file = std::fs::File::create("aigbona_juliet.txt").expect("create failed");
        file.write_all("Name: Aigbona Juliet\n".as_bytes()).expect("write failed");
        file.write_all("Qualification: B.Sc.\n".as_bytes()).expect("write failed");
        file.write_all("Department: Consulting\n".as_bytes()).expect("write failed");
        file.write_all(consulting.as_bytes()).expect("write failed");
        println!("\nData written to file.");

        let assurance = {"Services: \nAudit services, \nClimate change and sustainaibility services, \nFinancial accounting advisory services, 
        Forensic and integrity services, \nPrivate client audit experience, \nAccounting Link, \nAssurance."};
        let mut file = std::fs::File::create("akpevwe_iloka.txt").expect("create failed");
        file.write_all("Name: Akpevwe Iloka\n".as_bytes()).expect("write failed");
        file.write_all("Qualification: HND\n".as_bytes()).expect("write failed");
        file.write_all("Department: Assurance\n".as_bytes()).expect("write failed");
        file.write_all(assurance.as_bytes()).expect("write failed");
        println!("\nData written to file.");
    }
    }
    code_7();

    if code == 8{
        println!("");
        fn code_8() {

        let tax = {"Services: \nTax planning, \nTax function operations, \nTax policy and controversy, \nGlobal trade, \nTax accounting, \nTax compliance, \nTransaction tax."};
        let mut file = std::fs::File::create("adamu_sagamu.txt").expect("create failed");
        file.write_all("Name: Adamu Sagamu\n".as_bytes()).expect("write failed");
        file.write_all("Qualification: B.Sc.\n".as_bytes()).expect("write failed");
        file.write_all("Department: Tax\n".as_bytes()).expect("write failed");
        file.write_all(tax.as_bytes()).expect("write failed");
        println!("\nData written to file.");

        let people = {"Change management and experience, HR transformation, Integrated workforce mobility, Learning and development consulting, 
        Recognition and reward advisory, Workforce analytics, People and workforce."};
        let mut file = std::fs::File::create("gbenga_daniels.txt").expect("create failed");
        file.write_all("Name: Gbenga Daniels\n".as_bytes()).expect("write failed");
        file.write_all("Qualification: HND\n".as_bytes()).expect("write failed");
        file.write_all("Department: People and workforce\n".as_bytes()).expect("write failed");
        file.write_all(people.as_bytes()).expect("write failed");
        println!("\nData written to file."); 
    }
    }
    code_8();

    if code == 9{
        println!("");
        fn code_9() {

        let strategy = {"Services: \nStrategy consulting, \nCorporate and growth strategy, \nTransaction strategy and execution, 
        Restructuring and turnaround strategy, \nIndustry strategy, \nDigital business building, \nCommercial strategy."};
        let mut file = std::fs::File::create("ehis_ero.txt").expect("create failed");
        file.write_all("Name: Ehis Ero\n".as_bytes()).expect("write failed");
        file.write_all("Qualification: M.Sc.\n".as_bytes()).expect("write failed");
        file.write_all("Department: Strategy\n".as_bytes()).expect("write failed");
        file.write_all(strategy.as_bytes()).expect("write failed");
        println!("\nData written to file.");

        let transactions = {"Services: \nCorporate finance, \nDivestments and carve-outs, \nSustainability and ESG Services, \nM&A advisory, 
        M&A integration, \nM&A technology and tools, \nM&A advanced analytics."};
        let mut file = std::fs::File::create("maria_akinsola.txt").expect("create failed");
        file.write_all("Name: Maria Akinsola\n".as_bytes()).expect("write failed");
        file.write_all("Qualification: M.Sc.\n".as_bytes()).expect("write failed");
        file.write_all("Department: Transactions and corporate finance\n".as_bytes()).expect("write failed");
        file.write_all(transactions.as_bytes()).expect("write failed");
        println!("\nData written to file.");
    }
    }
    code_9();

fn main() {

    //Introduction
    println!("Welcome to Ernest & Young Global Limited.");
    println!("\nWe are a multinational professional services 
    partnership headquartered in London, England.");

    println!("Code 7: Aigbona Juliet - Consulting Department &\nAkpevwe Iloka - Assurance Department  ");
    println!("Code 8: Gbenga Daniels - People workforce &\nAdamu Sagamu - Tax ");
    println!("Code 9: Maria Akinsola - Transactions and corporate finance &\nEhis Ero - Strategy");
    println!("Which of the codes do you want to run: \nType 7 for code 7, 8 for code 8 and 9 for code 9");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let code:i32 = input.trim().parse().expect("Failed to input");

    if code == 7{ 
        code_7();
        println!("Success");
    }
    else if code == 8{
        code_8();
        println!("Success");
    }
    else if code == 9{
        code_9();
        println!("Success");
    }
    else {
        println!("Not a success");
    }

    println!("");
    code_7();
    code_8();
    code_9();
}
