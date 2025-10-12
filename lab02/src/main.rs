fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    let mut i = 0;

    while i < n {
        s.push(c);
        i += 1;
    }

    s
}

fn p1() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

fn add_chars_n_ref(s: &mut String, c: char, n: i32) {
    let mut i = 0;

    while i < n {
        s.push(c);
        i += 1;
    }
}

fn p2() {
    let mut s = String::from("");
    let mut i = 0;

    let mut_ref_to_s: &mut String = &mut s;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n_ref(mut_ref_to_s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

fn add_space(s: &mut String, n: i32) {
    let mut i = 0;

    while i < n {
        s.push(' ');
        i += 1;
    }
}

fn add_str(s: &mut String, n: &str) {
    s.push_str(n);
}

fn add_interger(s: &mut String, mut n: i32) {
    if n == 0 {
        s.push('0');
    }

    if n < 0 {
        s.push('-');
        n = -n;
    }

    let mut v: Vec<i32> = Vec::new();

    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }

    for i in (0..v.len()).rev() {
        s.push((b'0' + v[i] as u8) as char);
        if i % 3 == 0 && i != 0 {
            s.push('_');
        }
    }
}

fn add_float(s: &mut String, n: f64) {
    if n < 0.0 {
        s.push('-');
    }

    let mut interger_part: u32 = n as u32;

    let mut float_part: u32 = ((n - interger_part as f64) * 10_000.0) as u32;

    let mut v_int: Vec<u32> = Vec::new();
    let mut v_float: Vec<u32> = Vec::new();

    if interger_part == 0 {
        s.push('0');
    }

    while interger_part > 0 {
        v_int.push(interger_part % 10);

        interger_part /= 10;
    }

    if float_part == 0 {
        v_float.push(0);
    }

    while float_part > 0 {
        v_float.push(float_part % 10);

        float_part /= 10;
    }

    for i in (0..v_int.len()).rev() {
        s.push((b'0' + v_int[i] as u8) as char);
    }

    s.push('.');

    for i in (0..v_float.len()).rev() {
        s.push((b'0' + v_float[i] as u8) as char);
    }
}

fn p3() {
    let mut s: String = String::from("");
    let ref_to_s: &mut String = &mut s;

    add_space(ref_to_s, 40);
    add_str(ref_to_s, "I 💚\n");
    add_space(ref_to_s, 40);
    add_str(ref_to_s, "RUST.\n\n");
    add_space(ref_to_s, 4);
    add_str(ref_to_s, "Most");
    add_space(ref_to_s, 12);
    add_str(ref_to_s, "crate");
    add_space(ref_to_s, 6);
    add_interger(ref_to_s, 306437968);
    add_space(ref_to_s, 11);
    add_str(ref_to_s, "and");
    add_space(ref_to_s, 5);
    add_str(ref_to_s, "latest");
    add_space(ref_to_s, 9);
    add_str(ref_to_s, "is\n");

    add_space(ref_to_s, 9);
    add_str(ref_to_s, "downloaded");
    add_space(ref_to_s, 8);
    add_str(ref_to_s, "has");
    add_space(ref_to_s, 13);
    add_str(ref_to_s, "downloads");
    add_space(ref_to_s, 5);
    add_str(ref_to_s, "the");
    add_space(ref_to_s, 9);
    add_str(ref_to_s, "version");
    add_space(ref_to_s, 4);
    add_float(ref_to_s, 2.2038);

    println!("{s}");
}

fn main() {
    println!("Output problema 1: ");
    p1();
    println!("");
    println!("Output problema 2: ");
    p2();
    println!("");
    println!("Output problema 3: ");
    p3();
}
