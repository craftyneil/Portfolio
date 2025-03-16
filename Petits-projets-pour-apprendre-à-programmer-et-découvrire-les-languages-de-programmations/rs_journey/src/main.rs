#![allow(dead_code)]

fn main() {
}


/// Returns the factorial of a number.
///
/// # Examples
///
/// ```
/// let five = 5;
/// let result = factorial(five);
/// assert_eq!(120, result);
/// ```

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

///superate all the digits of a number into a vector
/// 
/// # Examples
///
///
///
///
///
///
fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn number_to_digit(n: u32) -> u32 {
    let result: u32 = number_to_vec(n)
                            .iter()
                            .fold(0, |acc, &x| acc + x);
    if result > 10 {
        return number_to_digit(result);
    }
    result
}