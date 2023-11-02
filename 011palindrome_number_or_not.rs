/*
PrepInsta Top 100 codes
program-11
input: You are given a natural number;
output: print whether the given number is palindrome or not;

note: Try to handle as many exceptional cases as possible.
*/

use std::io;

fn main()
{
    let mut number=String::new();

    let mut number:u32=loop{
        println!("Enter a natural number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number");
        match number.trim().parse(){
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter a valid natural number");
                number.clear();
                continue;
            }
        };
    };

    let original:u32=number;
    let mut reverse=0;
    while number!=0 {
        reverse=reverse*10+number%10;
        number/=10;
    }

    if original==reverse{
        println!("{} is a palindrome",original);
    }
    else{
        println!("{} is not a palindrome",original);
    }
}