use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

println!("Enter the customer's name: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");

println!("Enter the units comsumed: ");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let u: f32 = input2.trim().parse().expect("Not a valid number");

println!("Enter the rate per unit: ");
io::stdin().read_line(&mut input3).expect("Not a valid string");
let r: f32 = input3.trim().parse().expect("Not a valid number");

let electricity_bill: f32 = u * r;

if u > 0.0 && u < 100.0 {
	let electricity_bill: f32 = u * r;
	println!("The costumer's electricity bill is:{}", electricity_bill );
}
else if u > 101.0 && u < 300.0 {
	let electricity_bill: f32 = u * r;
	println!("The costumer's electricity bill is:{}", electricity_bill);
}
else if u > 301.0 {
	let electricity_bill: f32 = u * r;
	println!("The costumer's electricity bill is:{}",electricity_bill );
}
else if u > 500.0 {
	let electricity_bill: f32 = ( u * r ) + 5_000.0;
	println!("The costumer's electricity bill is {}", electricity_bill);
	println!("The costumer's name is {}", input1);
	println!("The units consumed {}", input2);
	println!("The rate per unit {}", input3);
  }
}
