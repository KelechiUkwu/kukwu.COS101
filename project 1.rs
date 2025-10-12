 fn main() {
 	let p: f64 = 520_000_000; // Principal
 	let r: f64 = 10.0;       // Rate (%)
 	let n: f64 = 5.0;       // Time (years)

 	// Compound interest formula:A =P * (1 + r/100)^t
 	let a = p * (1.0 + (r / 100.0)).powf(t);
 	println!("Amount is {:.2}", a);

 	let ci = a - p;
 	println!("Compound Interest is {:.2}", ci);

}