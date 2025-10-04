fn check_if_prime(number: i32) -> bool {
    let mut x = 2;

    if number % x == 0 && number != 2 {
        return false;
    }

    x = 3;

    while x * x < (number) {
        if number % x == 0 {
            return false;
        }

        x += 2;
    }

    true
}

fn p1() {
    let mut i = 0;

    while i <= 100 {
        if check_if_prime(i) {
            println!("Prime numbers are: {}", i);
        }
        i += 1;
    }
}

/////////////////

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn p2() {
    for i in 0..100 {
        for j in 0..100 {
            if i > 0 && j > 0 && gcd(i, j) == 1 {
                println!("{0} and {1} are coprime.", i, j);
            }
        }
    }
}

/////////

fn p3() {
    for i in 1..98 {
        println!("{} bottles of beer on the wall,", 100 - i);
        println!("{} bottles of beer.", 100 - i);
        println!("Take one down, pass it around.");
        println!("{} bottles of beer on the wall.", 99 - i);
        println!();
    }

    println!("{} bottles of beer on the wall,", 2);
    println!("{} bottles of beer.", 2);
    println!("Take one down, pass it around.");
    println!("{} bottle of beer on the wall.", 1);
    println!();

    println!("{} bottle of beer on the wall,", 1);
    println!("{} bottle of beer.", 1);
    println!("Take it down, pass it around.");
    println!("No more bottles of beer on the wall.");
    println!();
}

fn main() {
    p1();

    p2();

    p3();
}
