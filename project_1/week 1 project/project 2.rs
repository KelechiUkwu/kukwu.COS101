 fn main() {
 	let t: f64 = 450_000.00;
 	let m: f64 = 1_500_000.00;
 	let h: f64 = 750_000.00;
 	let d: f64 = 2_850_000.00;
 	let a: f64 = 250_000.00;
	 
	// sum
	let s = (2.0 * t) + m + (3.0 * h) + (3.0 * d) + a;

	println!("Total amount is {:.2}", s);

	let avg = s / 5.0;

	println!("Average amount is {:.2}", avg);
}