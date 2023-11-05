/*
PrepInsta Top 100 codes
program-17
input: You are given 2 numbers base and power.
constraints: base is a real number; power is an integer
output: print base^power;

note: Try to handle as many exceptional cases as possible.

Here there are many ways of solving this question
But I prefer to use inbuilt  "powf()" or "powi()" function

note: The powf function can only be used for float data types
If you want to use for integer data types then you must use powi() function
*/

use std::io;

fn main(){
    let mut base=String::new();
    let mut power=String::new();

    let base:f64=loop{
        println!("Enter base value: ");
        io::stdin()
            .read_line(&mut base)
            .expect("Failed to read base");
        match base.trim().parse(){
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Invalid input! Base value must be a floating point number");
                base.clear();
                continue;
            }
        };
    };

    let power:i32=loop{
        println!("Enter power value: ");
        io::stdin()
            .read_line(&mut power)
            .expect("Failed to read power");
        match power.trim().parse(){
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Invalid input! Power value must be an integer number");
                power.clear();
                continue;
            }
        };
    };

    println!("{} power {} is  {}",base,power,base.powi(power));
}