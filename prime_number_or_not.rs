/*
PrepInsta Top 100 codes
program-7
input: You are given a Natural number (Ex: 19);
output: print whether the input number is prime or not

note: Try to handle as many exceptional cases as possible.
*/

use std::io;
use std::process;

fn main()
{
    let mut number = String::new();
    
    let number:u32 = loop{
        println!("Enter a natural number");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number");
        match number.trim().parse() {
            Ok(num)=>{
                if num ==0 {
                    println!("Enter a natural number only");
                    number.clear();
                    continue;
                }
                break num;
            },
            Err(_)=>{
                number.clear();
                println!("Enter a natural number only");
                continue;
            }
        };
    };

    if number==1{
        println!("1 is neither prime nor composite");
        process::exit(0);
    }

    let mut i : u32=2;
    let mut no_of_factors: i32=0;
    while i*i <= number{
        if number%i == 0{
            no_of_factors+=1;
        }
        i+=1;
    }
    if no_of_factors>0{
        println!("{} is not a prime number",number);
    }
    else{
        println!("{} is a prime number",number);
    }
}