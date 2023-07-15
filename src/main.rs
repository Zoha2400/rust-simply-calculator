use std::io;

fn main()
{
    let mut a = String::new();
    let mut b = String::new();
    let mut symb = String::new();

    println!("a:");
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line!");

    println!("b:");
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line!");

    println!("type the operation symbol (+, -, *, /)");
    io::stdin()
        .read_line(&mut symb)
        .expect("Failed to read line!");

    let a: u32 = a.trim().parse().expect("It's not a number!");
    let b: u32 = b.trim().parse().expect("It's not a number!");


    if symb.trim() == "+" {println!("{}", (a + b))}
    else if symb.trim() == "-"{println!("{}", (a - b))}
    else if symb.trim() == "*"{println!("{}", (a * b))}
    else if symb.trim() == "/"{println!("{}", (a / b))}
    else{println!("Choose an existed operation!")};
}   