use std::io;


fn main() {
    println!("IDENTITY CARD");
    
    println!("Enter your Name: ");
    let mut fullName = String::new();
    io::stdin()
        .read_line(&mut fullName)
        .expect("failed to read name");
    fullName = fullName.trim().to_string();
    println!("Hello, {fullName}!");

    println!("Enter your Age: ");
    let mut hisAge = String::new();
    io::stdin()
        .read_line(&mut hisAge)
        .expect("failed to read age");
    hisAge = hisAge.trim().to_string();
    println!("Hello, {hisAge}!");
}
