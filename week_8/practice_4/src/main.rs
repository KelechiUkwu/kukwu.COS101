fn main() {

    //Name vector
    let name = vec!["Erin","Sam","Greg","Ade","Alo","Dan","Kin"];
    
    //Age vector
     let age = vec![18,19,30,40,60,20,34,56];

    println!("\nAge allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
        //iterating through i on the vector
        print!("{} is {} years old\n",name[i], age[i]);
    }
}
