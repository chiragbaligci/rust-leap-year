// Initializing a new function
fn is_leap(year: i32) -> bool {   //appropriate data type
    let factor = |x| year % x == 0;  // modulo operetor
    factor(4) && (!factor(100) || factor(400))  //checking
}
