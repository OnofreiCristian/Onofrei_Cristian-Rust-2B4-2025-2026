fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    } else if n < 2 || n.is_multiple_of(2) {
        return false;
    } else {
        let mut i = 3;

        while i * i < n {
            if n.is_multiple_of(i) {
                return false;
            }

            i += 2;
        }
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    if x == 65_535 {
        return None;
    }

    let mut y = x + 1;

    while y != u16::MAX {
        if is_prime(y as u32) {
            return Some(y);
        }

        y += 1;
    }

    None
}

fn pb1() {
    let mut i = next_prime(1);

    while i.is_some() {
        println!("{}\n", i.unwrap());

        i = next_prime(i.unwrap());
    }
}

fn checked_addition(a: u32, b: u32) -> u32 {
    if a > u32::MAX - b {
        panic!("Overflow occured during addition.");
    }

    a + b
}

fn checked_multiplication(a: u32, b: u32) -> u32 {
    if b != 0 && a > u32::MAX / b {
        panic!("Overflow occured during multiplication.");
    }

    a * b
}
fn pb2() {
    println!("{}", checked_addition(1, 1));

    println!("{}", checked_multiplication(1, 2));
}

////////////
#[derive(Debug)]
enum Error {
    OverflowAdd { a: u32, b: u32 },
    OverflowMul { a: u32, b: u32 },
}

fn checked_addition_2(a: u32, b: u32) -> Result<u32, Error> {
    if a > u32::MAX - b {
        Err(Error::OverflowAdd { a, b })
    } else {
        Ok(a + b)
    }
}

fn checked_multiplication_2(a: u32, b: u32) -> Result<u32, Error> {
    if a > u32::MAX / b && b != 0 {
        Err(Error::OverflowAdd { a, b })
    } else {
        Ok(a * b)
    }
}

fn pb3_1(a: u32, b: u32, c: u32) -> Result<u32, Error> {
    let sum = checked_addition_2(a, b)?;
    let mult = checked_multiplication_2(sum, c)?;

    Ok(mult)
}

fn pb3_2(a: u32, b: u32, c: u32) {
    match pb3_1(a, b, c) {
        Ok(n) => println!("{}", n),
        Err(e) => println!("Error! {:?}", e),
    }
}

///////

enum CharErrors {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharErrors> {
    if c.is_ascii_alphabetic() {
        return Ok(c.to_ascii_uppercase());
    } else {
        return Err(CharErrors::NotLetter);
    }
}

fn to_lowercase(c: char) -> Result<char, CharErrors> {
    if c.is_ascii_alphabetic() {
        return Ok(c.to_ascii_lowercase());
    } else {
        return Err(CharErrors::NotLetter);
    }
}

fn print_char(c: char) -> Result<(), CharErrors> {
    if c.is_ascii_hexdigit() {
        println!("{}", c);
        Ok(())
    } else {
        Err(CharErrors::NotPrintable)
    }
}

fn char_to_number(c: char) -> Result<u8, CharErrors> {
    if !c.is_ascii() {
        return Err(CharErrors::NotAscii);
    }

    if c.is_ascii_digit() {
        Ok(c as u8 - '0' as u8)
    } else {
        Err(CharErrors::NotDigit)
    }
}

fn char_to_number_hex(c: char) -> Result<u8, CharErrors> {
    if !c.is_ascii() {
        return Err(CharErrors::NotAscii);
    }

    if c.is_ascii_hexdigit() {
        if c as u8 - '0' as u8 <= 9 {
            Ok(c as u8 - '0' as u8)
        } else {
            let mut k = c;

            k = k.to_ascii_uppercase();

            Ok(10 + (k as u8 - 'A' as u8))
        }
    } else {
        Err(CharErrors::NotBase16)
    }
}

fn print_error(err: CharErrors) {
    match err {
        CharErrors::NotAscii => println!("Error! Character is not ascii!"),
        CharErrors::NotBase16 => println!("Error! Character is not in Base16!"),
        CharErrors::NotDigit => println!("Error! Character is not a digit!"),
        CharErrors::NotLetter => println!("Error! Character is not a letter!"),
        CharErrors::NotPrintable => println!("Error! Character is not a printable character!"),
    }
}

fn pb4(c: char) {
    match char_to_number_hex(c) {
        Ok(n) => println!("Problema 4 result: {}", n),
        Err(e) => print_error(e),
    }
}

//////
//I chose to make a very simple application that just checks if a number is even or odd;

fn is_even(n: u32) -> Option<bool> {
    if n.is_multiple_of(2) {
        Some(true)
    } else {
        None
    }
}

fn pb5(n: u32) {
    if is_even(n).is_some() {
        println!("The number {} is even!", n);
    } else {
        println!("The number {} is odd!", n);
    }
}

fn main() {
    pb1();
    pb2();
    pb3_2(4, 5, 6);
    pb4('A');
    pb5(5);
}
