/*
PrepInsta Top 100 codes
program-4
input: You are given two Natural Numbers 'number1' and 'number2';
constraint: number1<number2;
output: print sum of all integers in given range(inclusive);

note: Try to handle as many exceptional cases as possible.
*/

/*
Note:
This problem is solved by adding each natural number through a loop;
you can even solve them in multiple ways by using sum of n natural numbers formulae,
and apply this on both the extremes.
*/

use std::io;

fn main()
{
    let mut number1 = String::new();
    let mut number2 = String::new();

    let mut number1 : i32 = loop{
        println!("Enter number1: ");
        io::stdin()
            .read_line(&mut number1)
            .expect("Failed to read number1");
        match number1.trim().parse() {
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter a valid integer: ");
                number1.clear();
                continue;
            }
        };
    };

    let mut number2 : i32 = loop{
        println!("Enter number2: ");
        io::stdin()
            .read_line(&mut number2)
            .expect("Failed to read number2");
        match number2.trim().parse() {
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter a valid integer: ");
                number2.clear();
                continue;
            }
        };
    };

    if number1>number2 {
        let temp : i32=number1;
        number1=number2;
        number2=temp;
    }

    let mut sum:i32 = 0;
    for i in number1..number2+1
    {
        sum+=i;
    }
    println!("The sum of numbers in the range of [{},{}] is {}",number1,number2,sum);
}