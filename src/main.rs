mod math;
use math::to_eval::*;
use std::io;

fn main() {
   
    let mut formula = String::new();
    println!("digite a formula:");
    loop{
    io::stdin().read_line(&mut formula).expect("erro ao ler ");
    
    let response = to_eval(formula.trim());

    println!("result: {}", response);
    formula=String::new();
   }
    
}
