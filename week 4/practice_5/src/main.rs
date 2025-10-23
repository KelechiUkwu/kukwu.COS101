use std::io;
fn main() 
{
    let mut input = String::new();

    println!("\nEnter your height (in centimetres):");
    io::stdin().read_line(&mut input).expect("Not a variable string");
    let  height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are of average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {   
        println!("You are a dwarf");
    }
    else 
    {
        println!("Abnormal height");
    }
}
