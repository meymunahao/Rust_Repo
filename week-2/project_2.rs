fn main () {
	let qt:f64 = 2.00;
	let qh:f64 = 3.00;
	let qd:f64 = 3.00;
	let qm:f64 = 1.00;
	let qa:f64 = 1.00;
	let at:f64 = 450000.00;
	let ah:f64 = 750000.00;
	let ad:f64 = 2850000.00;
	let am:f64 = 1500000.00;
	let aa:f64 = 250000.00;


	// Sum and Average
	let s = at+ah+ad+am+aa;
	let q = qt+qh+qd+qm+qa;
	println!("Sum is {}", s);
	let a = (s)/q;
	println!("Average is {}", a);	
}

