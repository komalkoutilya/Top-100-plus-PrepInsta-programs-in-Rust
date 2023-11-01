/*
PrepInsta Top 100 codes
program-2
input: You are given an Integer;
output: print whether it is a Even or Odd number;

note: Try to handle as many exceptional cases as possible.
*/

use std::io;

fn main()
{
    let mut number = String :: new();

    let number : i32 = loop
    {
        println!("Enter an Integer: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read input");

        match number.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                number.clear();
                println!("Enter Integer values only!!");
                continue;
            }
        };
    };

    if number%2==0
    {
        println!("{} is an Even number",number);
    }
    else
    {
        println!("{} is an Odd number",number);
    }
}