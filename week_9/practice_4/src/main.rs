use std::io::Write;
use std::fs::OpenOptions;

fn main() {

    let _file = std::fs::File::create("data.txt").expect("create failed");
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write("\nHello Class".as_bytes()).expect("write failed");
    file.write("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
    println!("file append success");
}
