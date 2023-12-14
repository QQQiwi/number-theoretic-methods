use std::io;
use std::num;
// use rand::Rng;
use std::f64;
// use quadratic::jacobi;
// use rand::distributions::Uniform;


fn read_integer() -> i128 {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i128 = n.trim().parse().expect("invalid input");
    n
}


fn read_float() -> f32 {
    let mut num:f32=0.0;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    num = input.trim().parse().expect("Not a valid number");
    num
}


fn euclid_gcd_extended(a: i128, b: i128, _x: i128, _y: i128) -> (i128, i128, i128, i128) {
    if a == 0 {
        return (a, b, 0, 1);
    }
    else {
        let (_, d, x1, y1) = euclid_gcd_extended(b % a, a, 0, 0);
        let division = b / a;
        let x = y1 - division * x1;
        let y = x1;
        return (0, d, x, y)
    }
}


fn continued_fraction(mut a: i128, mut b: i128) -> Vec<i128> {
    let mut fraction: Vec<i128> = Vec::new();
    while b != 0 {
        fraction.push(a / b);
        let c = a;
        a = b;
        b = c % b;
    }
    fraction
}


fn print_array(arr: Vec<i128>) {
    print!("[");
    for i in 0..arr.len() {
        print!("{}", arr[i]);
        if i < arr.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}



fn mod_inverse(a: i128, n: i128) -> i128 {
    let fraction: Vec<i128> = Vec::new();
    let (_, g, x, _) = euclid_gcd_extended(a, n, 0, 0);
    if g != 1 {
        return -1;
    } else {
        let result = (x % n + n) % n;
        return result;
    }
}


fn is_prime(n: i128) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}


fn power_mod(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base.rem_euclid(modulus);
    let mut exp = exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base).rem_euclid(modulus);
        }
        exp = exp >> 1;
        base = (base * base).rem_euclid(modulus);
    }

    result.rem_euclid(modulus)
}


fn jacobi_symbol(mut a: i128, mut n: i128) -> i128 {
    let mut t = 1;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let n_mod_8 = n % 8;
            if n_mod_8 == 3 || n_mod_8 == 5 {
                t = -t;
            }
        }

        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
            t = -t;
        }

        a %= n;
    }

    if n == 1 {
        return t;
    } else {
        return 0;
    }
}


// fn baby_step_giant_step() -> () {
//     println!("Введите a: ");
//     let mut a = read_integer();
//     println!("Введите B: ");
//     let mut p = read_integer() as f64;
//     println!("Введите b: ");
//     let mut b = read_integer() as f64;

//     let big_b = p - 1.0;
//     let r = (big_b.sqrt()) as i128 + 1;
//     // println!("{}", r);
//     let mut pairs: Vec<(i128, i128)> = vec![];
//     for k in 1..r {
//         pairs.push((k, power_mod(a as u64, k as u64, p as u64) as i128))
//     }
//     pairs.sort_by(|a, b| a.1.cmp(&b.1));
//     // println!("{:?}", pairs);
//     let inv_powered_a = mod_inverse(power_mod(a as u64, r as u64, p as u64) as i128,
//                                     p as i128);
//     for i in 0..r {
//         let mut result = power_mod(inv_powered_a as u64, i as u64, p as u64) * (b as u64);
//         result = result.rem_euclid(p as u64);
//         for k in 0..(r - 1) {
//             if pairs[k as usize].1 == result as i128 {
//                 if power_mod(a as u64, (k + r * i) as u64, p as u64) == b as u64 {
//                     println!("Значение x = {}", k + r * i);
//                     return ();
//                 }
//                 else if power_mod(a as u64, (k + 1 + r * i) as u64, p as u64) == b as u64 {
//                     println!("Значение x = {}", k + 1 + r * i);
//                     return ();
//                 }
//             }
//         }
//     }
//     println!("Значение a не является образующим элементом.");
// }


fn baby_step_giant_step() -> () {
    println!("Введите a: ");
    let mut alpha = read_integer() as i64;
    println!("Введите B: ");
    let mut n = read_integer() as i64;
    println!("Введите b: ");
    let mut beta = read_integer() as i64;

    let m = (n as f64).sqrt() as i64 + 1;

    let alpha_inv_m = power_mod(alpha as u64, (n - m - 1) as u64, n as u64);

    let mut hash_table = std::collections::HashMap::new();

    for j in 0..m {
        let value = power_mod(alpha as u64, j as u64, n as u64);
        hash_table.insert(value, j);
    }

    for i in 0..m {
        let value = (beta * (power_mod(alpha_inv_m as u64, i as u64, n as u64)) as i64).rem_euclid(n);

        if let Some(j) = hash_table.get(&(value as u64)) {
            let mut x = i * m + *j;
            if x != 0
            {
                println!("Значение x = {}", i * m + *j);
                return ()
            }
        }
    }

    println!("Значение a не является образующим элементом.");
    return ()
}


fn rho_method() -> () {
    println!("Введите a: ");
    let mut a = read_integer();
    println!("Введите m: ");
    let mut p = read_integer() as f64;
    println!("Введите b: ");
    let mut b = read_integer() as f64;

    
}


fn main() {
    println!("Выберите метод разложения:");
    println!("1. метод Гельфонда-Шенкса");
    println!("2. p-метод Полларда");
    println!("3. Индекс-метод");
    
    let chosen_method = read_integer();
    match chosen_method {
        1 => baby_step_giant_step(),
        2 => rho_method(),
        _ => println!("Введено неверное число!"),
    }

}
