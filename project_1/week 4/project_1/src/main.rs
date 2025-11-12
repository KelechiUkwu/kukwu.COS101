use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    let y: f32 = b * b - 4.0 * a * c;        // discriminant 

    if y > 0.0 {
        let root1: f32 = (- b + y.sqrt()) / (2.0 * a);
        let root2: f32 = (- b - y.sqrt()) / (2.0 * a);
        println!("There are two real roots: {:.2} and {:.2}", root1, root2);
    } else if y == 0.0 {
        let root: f32 = -b / (2.0 * a);
        println!("There is only one real root: {:.2}", root);
    } else {
    println!("There are no real roots");
    }
}