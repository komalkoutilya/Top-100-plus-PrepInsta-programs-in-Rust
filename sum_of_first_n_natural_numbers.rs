/*
PrepInsta Top 100 codes
program-3
input: You are given a Natural Number num;
output: print sum of first 'number' natural numbers;

note: Try to handle as many exceptional cases as possible.
*/

/*
Note:
This problem is solved by adding each natural number through a loop;
You can also solve using formulae: (n*(n+1))/2
*/

use std::io;

fn main()
{
    let mut number=String::new();
    let number : u32 = loop
    {
        println!("Enter a Natural Number: ");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read a number");

        match number.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                println!("Enter a valid natural number only");
                number.clear();
                continue;
            }
        };
    };

    let mut sum : u32 = 0;
    for i in 1..(number+1)
    {
        sum+=i;
    }
    println!("The sum of first {} natural numbers: {}",number,sum);
}