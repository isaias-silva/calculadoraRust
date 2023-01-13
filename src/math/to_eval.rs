use eval::eval;
pub fn to_eval(formula: &str) -> String {
 
    let sum = eval(formula).expect("error ao converter formula");

    return sum.to_string();
}
