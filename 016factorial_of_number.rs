/*
PrepInsta Top 100 codes
program-16
input: You are given a non-negative number n;
output: print factorial of n (i.e n!)

note: Try to handle as many exceptional cases as possible.

Here there are many ways of solving this question
But I prefer to solve it using recursion

Note here u8 is used for the input so max possible value is 255.
and to store factorial we used u128 as it can accomodate more larger numbers without any issue.
*/

use std::io;

fn main()
{
    let mut number=String::new();

    let number:u8=loop{
        println!("Enter a non-negative number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number");
        match number.trim().parse() {
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Invalid input");
                number.clear();
                continue;
            }
        };
    };
    println!("The factorial of {} is {}",number,factorial(number));
}

fn factorial(number:u8)->u128{
    if number==0{
        return 1_u128;
    }
    (number as u128*factorial(number-1)) as u128
}