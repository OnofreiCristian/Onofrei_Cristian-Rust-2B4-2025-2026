use std::{fs, io};

fn longest_line_by_chr_length(path: &str) -> Result<(String, usize), io::Error> {
    let s = fs::read_to_string(path)?;

    let mut longest_string_chr = String::new();
    let mut longest_string_byt: usize = 0;

    for line in s.lines() {
        let byte_len = line.len();
        let char_count = line.chars().count();

        if char_count > longest_string_chr.chars().count() {
            longest_string_chr = line.to_string();
            longest_string_byt = byte_len;
        }
    }

    //fs::write("output.txt", &s)?;

    Ok((longest_string_chr, longest_string_byt))
}

fn longest_line_by_byt_length(path: &str) -> Result<(String, usize), io::Error> {
    let s = fs::read_to_string(path)?;

    let mut longest_string_chr = String::new();
    let mut longest_string_byt: usize = 0;

    for line in s.lines() {
        let byte_len = line.len();

        if byte_len > longest_string_byt {
            longest_string_chr = line.to_string();
            longest_string_byt = byte_len;
        }
    }

    //fs::write("output.txt", &s)?;

    Ok((longest_string_chr, longest_string_byt))
}

fn pb1() {
    let str_example_1 = longest_line_by_chr_length("src/text.txt");

    match str_example_1 {
        Ok((_v, _u)) => println!(
            "Read from file!
        The line with the most characters is:
        {0} {1}",
            _v, _u
        ),
        Err(_e) => println!("Error!"),
    }

    let str_example_2 = longest_line_by_byt_length("src/text.txt");

    match str_example_2 {
        Ok((_v, _u)) => println!(
            "Read from file!
        The line with the most bytes is:
        {0} {1}",
            _v, _u
        ),
        Err(_e) => println!("Error!"),
    }
}

fn rot_13(input: &str) -> Result<String, &str> {
    let mut output = String::new();

    for ch in input.chars() {
        if !ch.is_ascii() {
            return Err("Error! Strnig is not ASCII.");
        }

        if ch.is_ascii_alphabetic() {
            let new_ch: char;

            if ch.is_ascii_uppercase() {
                if ch <= 'M' {
                    new_ch = (ch as u8 + 13) as char;
                } else {
                    new_ch = (ch as u8 - 13) as char;
                }

                output.push(new_ch);
            } else {
                if ch <= 'm' {
                    new_ch = (ch as u8 + 13) as char;
                } else {
                    new_ch = (ch as u8 - 13) as char;
                }

                output.push(new_ch);
            }
        } else {
            output.push(ch);
        }
    }

    Ok(output)
}

fn pb2() {
    let encrypt = rot_13("hello please CRYPT this.");

    match encrypt {
        Ok(result) => println!("Encrypted message:\n{}", result),
        Err(error) => println!("{}", error),
    }
}

fn replace_abbr(path: &str) -> Result<String, io::Error> {
    let mut s = fs::read_to_string(path)?;

    s = s.replace("pt", "pentru");
    s = s.replace("ptr", "pentru");
    s = s.replace("dl", "domnul");
    s = s.replace("dna", "doamna");

    Ok(s)
}

fn pb3() {
    let abbreviate = replace_abbr("src/b3.txt");

    match abbreviate {
        Ok(_str) => println!("{}", _str),
        Err(_error) => println!("Error! Coulnd't read from file."),
    }
}

fn read_from_host(path: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(path)?;

    for line in contents.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        let line_split: Vec<&str> = trimmed_line.split_whitespace().collect();

        if line_split.len() >= 2 {
            let ip_address = line_split[0];
            let hostname = line_split[1];
            println!("{} => {}", hostname, ip_address);
        }
    }

    Ok(())
}

fn pb4() {
    let result = read_from_host("C:\\Users\\Cris\\Documents\\hosts.txt");

    match result {
        Ok(()) => println!("Sucessfully read from file!"),
        Err(_error) => println!("Error! Failed at reading from file."),
    }
}

fn main() {
    pb1();
    pb2();
    pb3();
    pb4();
}
