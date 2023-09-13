use std::io;

fn read_integer() -> i32 {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    n
}


fn euclid_gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    };
    let r = a % b;
    return euclid_gcd(b, r);
}


fn euclid_gcd_extended(a: i32, b: i32, _x: i32, _y: i32) -> (i32, i32, i32, i32) {
    if a == 0 {
        return (a, b, 0, 1);
    }
    else {
        let (_, d, x1, y1) = euclid_gcd_extended(b % a, a, 0, 0);
        let x = y1 - (b / a) * x1;
        let y = x1;
        return (0, d, x, y)
    }
}


fn euclid_gcd_binary(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if a == b {
        return a;
    }

    if a == 1 || b == 1 {
        return 1;
    }

    if a % 2 == 0 && b % 2 == 0 {
        return 2 * euclid_gcd_binary(a / 2, b / 2);
    }

    if a % 2 == 0 && b % 2 != 0 {
        return euclid_gcd_binary(a / 2, b);
    }

    if a % 2 != 0 && b % 2 == 0 {
        return euclid_gcd_binary(a, b / 2);
    }

    if a % 2 != 0 && b % 2 != 0 && b > a {
        return euclid_gcd_binary((b - a) / 2, a);
    }

    return euclid_gcd_binary((a - b) / 2, b);
}


fn solve_euclid() {
    println!("Write a:");
    let a = read_integer();

    println!("Write b:");
    let b = read_integer();

    println!("{}", euclid_gcd(a, b));
    println!("{:?}", euclid_gcd_extended(a, b, 0, 0));
    println!("{}", euclid_gcd_binary(a, b));    
}



fn main() {
    solve_euclid()
}
