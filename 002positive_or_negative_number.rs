/*
PrepInsta Top 100 codes
program-1
input: You are given a number;
output: print whether it is a positive or negtive number;

note: Try to handle as many exceptional cases as possible.
*/

use std::io;

fn main()
{
    let mut number = String::new();
    let number:f64 = loop{
        println!("Enter a numerical value: ");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number");

        match number.trim().parse() {
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter numerical values only");
                number.clear();
                continue;
            }
        };
    };

    if number>0.0
    {
        println!("{} is a positive number",number);
    }
    else if number<0.0
    {
        println!("{} is a negative number",number);
    }
    else
    {
        println!("{} is neither positive nor negative",number);
    }
}