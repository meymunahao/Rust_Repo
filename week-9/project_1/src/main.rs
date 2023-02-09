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
