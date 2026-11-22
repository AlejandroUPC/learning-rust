use std::io;

fn main() {
    let x = 5;
    let x = 5 + 1;

    {
        let x = x * 2;
        println!("The value of inner scope is {x}");
    }
    println!("Outside is {x}");
    let spaces = "     ";
    let spaces = spaces.len();
    println!("IT is {spaces}.");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let a = [1, 2, 3, 4, 5];

    println!("Add index of array");

    let mut idx = String::new();
    io::stdin().read_line(&mut idx).expect("Failed");
    let idx: usize = idx.trim().parse().expect("cant parse");
    let element = a[idx];

    println!("Value of {idx} is {element}");
}
