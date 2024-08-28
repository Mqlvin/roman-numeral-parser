use thiserror::Error;

#[derive(Error, Debug)]
pub enum RomanNumeralError {
    #[error("Invalid roman numeral character")]
    BadCharacter,
    #[error("Error parsing roman numeral string")]
    BadString,
}

fn lookup_numeral(numeral: &char) -> Result<u32, RomanNumeralError> {
    match *numeral {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => Err(RomanNumeralError::BadCharacter)
    }
}

fn parse(num: &str) -> Result<u32, RomanNumeralError> {
    let mut total: u32 = 0;
    let mut last_num_val: u32 = u32::MAX; // this is only for the initial iteration
    for ch in num.chars() {
        let num_val = match lookup_numeral(&ch) {
            Ok(val) => val,
            Err(err) => { return Err(err); }
        };

        total += num_val;

        if num_val > last_num_val {
            total -= last_num_val * 2;
        }

        last_num_val = num_val;
    }

    Ok(total)
}

fn main() {
    let val = parse("CXXXIV");
    println!("The value of CXXXIV is {}", val.expect("parse error"));
}
