use std::cmp::Ordering;
use std::io;
fn main(){

    println!("WELCOME TO MY NEW GAME! HAVE FUN!");
    
loop{
    println!("Insert your first number:");
    let mut number_1 = String::new();
    io::stdin().read_line(& mut number_1).expect("Error");
    let a : i32 = match number_1.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("Insert your second number:");
    let mut number_2= String::new();
    io::stdin().read_line(& mut number_2).expect("Error");
    let b : i32 = match number_2.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if a >= 100 {
        println!("Fail! 
Your first number needs to be smaller.");
    }

    if b >= 100 {
        println!("Fail!
Your second number needs to be smaller.");
    }
    if a == 404 {
        println!("Error code. Game over!");
        break;
    }
    if b == 404 {
        println!("Error code. Game over!");
        break;
    }

    match number_1.cmp(&number_2) {

        Ordering::Greater => println!("First number is greater than second"),
        Ordering::Less => println!("First number is smaller than second"),
        Ordering::Equal => {
            println!("Both numbers are equal");
        }

    };
    match number_2.cmp(&number_1) {

        Ordering::Greater => println!("Second number is greater than first."),
        Ordering::Less => println!("Second number is smaller than first."),
        Ordering::Equal => {
            println!("Game over!");
            break;
        }
    }
}
}