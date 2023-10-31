/*
PrepInsta Top 100 codes
program-6
input: You are given an year (Ex: 2023);
output: print whether the input year is Leap Year or not;

note: Try to handle as many exceptional cases as possible.
*/

use std::io;

fn main()
{
    let mut year = String::new();

    let year:u32 = loop{
        println!("Enter year: ");
        io::stdin()
            .read_line(&mut year)
            .expect("Failed to read year");
        match year.trim().parse(){
            Ok(num)=>{
                break num;
            },
            Err(_)=>{
                year.clear();
                println!("Enter a valid year");
                continue;
            }
        };
    };

    if year%4 == 0{
        if year % 100 == 0 && year % 400 != 0{
            println!("{} is not a leap year",year);
        }
        else{
            println!("{} is a leap year",year);
        }
    }
    else{
        println!("{} is not a leap year",year);
    }
}