pub fn roman_to_int(s: String) -> i32 {
    let mut integer: i32 = 0;
    let mut counter: usize = 0;

    //Get the String length
    let length = s.chars().count();
    
    //Loop through the String
    while counter < length  {
        let i = s.chars().nth(counter);
        if i == Some('I'){
            if s.chars().nth(counter+1) == Some('V') {
                integer += 4;
                counter += 2;
                continue
            }
            else if s.chars().nth(counter+1) == Some('X') {
                integer += 9;
                counter += 2;
                continue
            }
            else {
                integer += 1;
                counter += 1;
                continue
            }
        }

        if i == Some('X') {
            if s.chars().nth(counter+1) == Some('L') {
                integer += 40;
                counter += 2;
                continue
            }
            else if s.chars().nth(counter+1) == Some('C') {
                integer += 90;
                counter += 2;
                continue
            }
            else {
                integer += 10;
                counter += 1;
                continue
            }
        }

        if i == Some('C') {
            if s.chars().nth(counter+1) == Some('D') {
                integer += 400;
                counter += 2;
                continue
            } else if s.chars().nth(counter+1) == Some('M') {
                integer += 900;
                counter += 2;
                continue
            } else {
                integer += 100;
                counter += 1;
                continue
            }
        }

        if i == Some('V') {
            integer += 5;
            counter += 1;
            continue
        }
        if i == Some('L') {
            integer += 50;
            counter += 1;
            continue
        }
        if i == Some('D') {
            integer += 500;
            counter += 1;
            continue
        }

        if i == Some('M') {
            integer += 1000;
            counter += 1;
            continue
        }

        
        if i != Some('I') || i != Some('V') || i != Some('X') || i != Some('L') || i != Some('C') || i != Some('D') || i != Some('M') {
            integer = 0;
            println!("Please enter a valid number.");
            break;
        }

        counter += 1;
    }  
    return integer;
}
    

fn main() {
    
    println!("III = {}",roman_to_int("III".to_string()));
    println!("LVII = {}",roman_to_int("LVIII".to_string()));
    println!("MCMXCIV = {}",roman_to_int("MCMXCIV".to_string()));
}