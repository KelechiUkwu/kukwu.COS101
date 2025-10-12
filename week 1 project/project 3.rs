 fn main() {
	let p:f64 = 510_000.00; // Principal
	let r:f64 = 5.0;       // Rate (%)
	let t:f64 = 3.0;      // Time (years)

	// Compound Interest formula:A = P * (1 + r/100)^t
	let a = p * (1.0 + (r / 100.0)).powf(t);
	println!("Amount is {:.2}", a);

 	let ci = a - p;
 	println!("Compound Interest is {:.2}", ci);
}