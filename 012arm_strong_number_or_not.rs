/*
PrepInsta Top 100 codes
program-12
input: You are given a natural number (Ex: 2991);
output: print whether the given number is armstrong number or not;

note: Try to handle as many exceptional cases as possible.
*/
use std::io;

fn main()
{
    let mut number=String::new();

    let number:u32=loop{
        println!("Enter a natural number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number");
        match number.trim().parse() {
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                println!("Enter a valid natural number only");
                number.clear();
                continue;
            }
        };
    };

    if is_armstrong(number) {
        println!("{} is an Armstrong number",number);
    }
    else{
        println!("{} is not an Armstrong number",number);
    }
}

fn is_armstrong(num:u32)->bool{
    let mut sum:u32=0;
    let mut len:u32=0;
    let mut original1:u32=num;
    let mut original2:u32=num;

    while original1!=0 {
        len+=1;
        original1/=10;
    }
    
    while original2!=0 {
        let mut rem=1;
        let mut i:u32=0;
        while i<len {
            rem=rem*(original2%10);
            i+=1;
        }
        sum+=rem;
        original2/=10;
    }

    if num==sum {
        return true;
    }
    return false;
}