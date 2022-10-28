 fn main() {
 	let p:f64 = 520000000.00;
 	let r:f64 = 0.100;
 	let t:f64 = 5.00;

 	// compound interest
 	let a = p * (1.0 + (r / 100.0)) * t;
	println!("Amount is {}", a);
	let ci = a - p; 
	println!("Compound Interest is {}", ci);

}