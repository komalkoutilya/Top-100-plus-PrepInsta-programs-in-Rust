/*
PrepInsta Top 100 codes
program-10
input: You are given a natural number (Ex: 2991);
output: print the reverse of that number (Ex: 1992)

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
        match number.trim().parse() {
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter valid natural numbers only");
                number.clear();
                continue;
            }
        };
    };

    let store_number:u32=number;
    let mut reverse:u32=0;

    while number!=0
    {
        reverse=reverse*10+number%10;
        number/=10;
    }
    println!("The value of reverse of {} is {}",store_number,reverse);
}