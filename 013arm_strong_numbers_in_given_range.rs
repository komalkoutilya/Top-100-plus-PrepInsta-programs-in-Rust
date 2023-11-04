/*
PrepInsta Top 100 codes
program-13
input: You are given 2 integers integer2>=integer1;
output: print all the numbers in the range which are supposed to be an armstrong number;

note: Try to handle as many exceptional cases as possible.
*/

use std::io;

fn main()
{
    let mut number1=String::new();
    let mut number2=String::new();

    let number1:u32=loop{
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
                println!("Enter a valid natural number");
                continue;
            }
        };
    };

    let number2:u32=loop{
        println!("Enter number2: ");
        io::stdin()
            .read_line(&mut number2)
            .expect("Failed to read number2");
        match number2.trim().parse(){
            Ok(num)=>{
                if num<number1 {
                    println!("number2 must be greater than or equal to number1 ({})",number1);
                    continue;
                }
                break num;
            },
            Err(_)=>{
                number2.clear();
                println!("Enter a valid natural number");
                continue;
            }
        };
    };

    armstrong_numbers_in_given_range(number1,number2);
}

fn armstrong_numbers_in_given_range(num1:u32,num2:u32){
    let mut noa:u32=0;
    for i in num1..num2+1 {
        if is_armstrong(i) {
            if noa==0 {
                println!("The Armstrong numbers in the range [{},{}] are: ",num1,num2);
            }
            println!("{} is an Armstrong number",i);
            noa+=1;
        }
    }
    if noa==0 {
        println!("There are no Armstrong numbers in the range [{},{}]",num1,num2);
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