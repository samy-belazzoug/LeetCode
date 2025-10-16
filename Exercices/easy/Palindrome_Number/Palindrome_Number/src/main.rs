pub fn is_palindrome(x: i32) -> bool {
    //Given an integer x, return true if x is a palindrome, and false otherwise.
    //let constraint = u32::pow(a, 31);
    /*As the number is an i32 and not a u32,
     we don't need to check if -2**31 <= x <= 2**31 - 1, it will always be    
    */
    let x_string = x.to_string();
    let mut result = String::new();
    for c in x_string.chars().rev() {
        result.push(c);
    }
    return x_string == result;
} 

fn main() {
    println!("Hello,World!");
    println!("121 : {}",is_palindrome(121));
    println!("-121 : {}",is_palindrome(-121));
    println!("10 : {}",is_palindrome(10));
}
