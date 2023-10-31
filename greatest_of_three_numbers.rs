/*
PrepInsta Top 100 codes
program-5
input: You are given three Integer Numbers 'number1' , 'number2' and 'number3';
output: print greatest among 3 integers;

note: Try to handle as many exceptional cases as possible.
*/

use std::io;

fn main()
{
    let mut number1 = String :: new();
    let mut number2 = String :: new();
    let mut number3 = String :: new();

    let number1 : i32 = loop{
        println!("Enter number1: ");
        io::stdin()
            .read_line(&mut number1)
            .expect("Failed to read number1");
        match number1.trim().parse(){
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter a valid integer");
                number1.clear();
                continue;
            }
        };
    };

    let number2 : i32 = loop{
        println!("Enter number2: ");
        io::stdin()
            .read_line(&mut number2)
            .expect("Failed to read number2");
        match number2.trim().parse(){
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter a valid integer");
                number2.clear();
                continue;
            }
        };
    };

    let number3 : i32 = loop{
        println!("Enter number3: ");
        io::stdin()
            .read_line(&mut number3)
            .expect("Failed to read number3");
        match number3.trim().parse(){
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter a valid integer");
                number3.clear();
                continue;
            }
        };
    };

    if number1 > number2 && number1 > number3
    {
        println!("{} is the largest among three numbers",number1);
    }
    else if number2 > number1 && number2 > number3
    {
        println!("{} is the largest among three numbers",number2);
    }
    else
    {
        println!("{} is the largest among three numbers",number3);
    }
}