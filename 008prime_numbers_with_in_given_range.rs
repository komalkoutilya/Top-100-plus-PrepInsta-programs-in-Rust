/*
PrepInsta Top 100 codes
program-8
input: You are given two integers (Ex: 19);
output: print all the prime numbers in the inclusive range [number1,number2] where number1<=number2

note: Try to handle as many exceptional cases as possible.
*/

use std::io;

fn main()
{
    let mut number1=String::new();

    let number1:i32=loop{
        println!("Enter number1: ");
        io::stdin()
            .read_line(&mut number1)
            .expect("Failed to read number1");
        match number1.trim().parse(){
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                number1.clear();
                println!("Enter a valid integer");
                continue;
            }
        };
    };

    let mut number2=String::new();

    let number2:i32=loop{
        println!("Enter number2: ");
        io::stdin()
            .read_line(&mut number2)
            .expect("Failed to read number2");
        match number2.trim().parse(){
            Ok(num)=>{
                if num<number1{
                    println!("number2 must be greater than or equal to {}",number1);
                    number2.clear();
                    continue;
                }
                break num;
            },
            Err(_)=>{
                number2.clear();
                println!("Enter a valid integer");
                continue;
            }
        };
    };

    let mut current:i32=number1;
    if current<2{
        current=2;
    }

    let mut no_of_primes:i32=0;
    while current<=number2{
        let mut no_of_factors:i32=0;
        let mut counter=2;
        while counter*counter<=current {
            if current%counter==0{
                no_of_factors+=1;
            }
            counter+=1;
        }
        if no_of_factors==0{
            if no_of_primes==0{
                println!("The prime numbers in [{},{}] are ",number1,number2);
            }
            println!("{} is prime number",current);
            no_of_primes+=1;
        }
        current+=1;
    }
    if no_of_primes==0{
        println!("There are no prime numbers in[{},{}]",number1,number2);
    }
}