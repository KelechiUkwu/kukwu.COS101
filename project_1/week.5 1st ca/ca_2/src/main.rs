use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();

    println!("Enter the principal,p: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the rate,r: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let r: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the time,t: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let t: f32 = input3.trim().parse().expect("Not a valid number");

    let mut a: f32 = p * ( 1.0 + ( r / 100.0 )).powf(t);

    if a > 0.0 {
    	let li: f32 = a - p ;
    	println!("The local interest is {}", li);
    }
    //To calculate for another borrower
    println!("Do you want to calculate for another borrower?:(Y / N)");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    }
    
    
