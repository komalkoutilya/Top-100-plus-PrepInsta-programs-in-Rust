/*
PrepInsta Top 100 codes
program-18
input: You are given a Non-negative integer.
output: Print all the non-negative factors of given number.

note: Try to handle as many exceptional cases as possible.

Here there are many ways of solving this question
But I prefer to solve it by iterating till the square root of the number.
*/

use std::io;

fn main(){
    let mut number=String::new();

    let number:u64=loop{
        println!("Enter a number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number");
        match number.trim().parse() {
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter a valid Non-negative Integer");
                number.clear();
                continue;
            }
        };
    };

    println!("The factors of {} are: {}",number,factors(number));
}

fn factors(num:u64)->String{
    let mut factors="1, ".to_string();
    factors.push_str(&num.to_string());
    let mut i:u64=2;
    while i*i<=num {
        if num%i==0{
            factors.push_str(", ");
            factors.push_str(&i.to_string());
            factors.push_str(", ");
            factors.push_str(&(num/i).to_string());
        }
        i+=1;
    }
    factors
}