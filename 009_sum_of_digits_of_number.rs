/*
PrepInsta Top 100 codes
program-9
input: You are given a natural number (Ex: 1991);
output: print the sum of individual digits of given natural number;

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
                number.clear();
                println!("Enter valid Natural Number");
                continue;
            }
        };
    };
    let store_number:u32=number;
    let mut sum:u32 = 0;
    while !(number==0) {
        sum+=number%10;
        number/=10;
    }
    
    println!("The sum of digits of {} is {}",store_number,sum);
}