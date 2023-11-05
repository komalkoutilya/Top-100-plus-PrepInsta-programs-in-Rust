/*
PrepInsta Top 100 codes
program-15
input: You are given a natural number n;
output: print nth term of fibonacci series.

Note: In my convention...
t0=0; t1=1; t2=1; t3=2; t4=3; t5=5; t6=8; t7=13;...
Aim: find tn where n>=0
note: Try to handle as many exceptional cases as possible.

Here there are many ways of solving this question
But the most straight-forward method is: use formula: 
*/

use std::io;

fn main()
{
    let mut term=String::new();

    let term:u32=loop{
        println!("Enter number of term: ");
        io::stdin()
            .read_line(&mut term)
            .expect("Failed to read term value");
        match term.trim().parse()
        {
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Your input must be a non-negative integer");
                term.clear();
                continue;
            }
        };
    };

    println!("The {} term of the Fibonacci series is: {}",term,fibonacci_nth_term(term));
}

fn fibonacci_nth_term(t:u32)->i64 {
    /*The formula of tn= (((1+5^(1/2))/2)^n-((1-5^(1/2))/2)^n)/5^(1/2)*/
    
    let t:f32 = t as f32;
    return ((((1.0+5_f32.sqrt())/2.0).powf(t)-((1.0-5_f32.sqrt())/2.0).powf(t))/5_f32.sqrt()) as i64;
}