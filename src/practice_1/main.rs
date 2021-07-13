use std::io;

fn main() {
    let mut line1 = String::new();
    io::stdin().read_line(&mut line1).expect("Failed to read line");
    let line1: u32 = line1.trim().parse().expect("Please type a number");

    let mut line2 = String::new();
    io::stdin().read_line(&mut line2).expect("Failed to read line");
    let v: Vec<u32> = line2.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();

    let mut line3 = String::new();
    io::stdin().read_line(&mut line3).expect("Failed to read line");

    println!("{} {}", line1 + v[0] + v[1], line3)
}