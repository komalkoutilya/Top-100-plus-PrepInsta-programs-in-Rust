/*
PrepInsta Top 100 codes
program-14
input: You are given a natural number n;
output: print all the first n terms of  fibonacci series.

Note: In my convention...
t0=0; t1=1; t2=1; t3=2; t4=3; t5=5; t6=8; t7=13;...

note: Try to handle as many exceptional cases as possible.
*/

use std::io;

fn main()
{
    let mut terms=String::new();

    let terms:u32=loop{
        println!("Enter number of terms: ");
        io::stdin()
            .read_line(&mut terms)
            .expect("Failed to read number of terms");

        match terms.trim().parse(){
            Ok(num)=>{
                if num==0{
                    println!("0 is a bad input for number of terms");
                    continue;
                }
                break num;
            },
            Err(_)=>{
                terms.clear();
                println!("Enter a valid natural number only");
                continue;
            }
        };
    };
    print!("The {} terms of fibonacci series: ",terms);
    fibonacci_series(terms);
}

fn fibonacci_series(terms:u32){
    let mut a:u32=0;
    let mut b:u32=1;
    let mut temp:u32;

    for _i in 1..terms+1{
        print!("{}\t",a);
        temp=a;
        a=b;
        b=temp+a;
    }
}