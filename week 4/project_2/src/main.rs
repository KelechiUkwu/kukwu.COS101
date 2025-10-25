use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Is the employee's experienced (T/F): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let input1: String = input1.trim().parse().expect("Not a valid number");
   
    println!("Enter the employes's age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 40 && input1 == "T"{
        println!("The incentive of the employee is 1_560_000");
    }
    else if age >= 30 && age < 40 && input1 == "T"{
        println!("The incentive of the employee is 1_480_000");
    } 
    else if age < 28 && input1 == "T" {  
        println!("The incentive of the employee is 1_300_000 per month");
    } 
    else if input1 == "F" {
        println!("The incentive of the employee is 100_000");
    }

}
