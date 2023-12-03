use std::io;
use std::num;
use rand::Rng;
use gcd::Gcd;
use std::f64;
// use quadratic::jacobi;
use rand::distributions::Uniform;
use std::collections::HashSet;

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


fn rho_method(n: u128) -> u128 {
    if n == 1 {
        return 1;
    }

    if n % 2 == 0 {
        return 2;
    }

    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(2..n);
    let mut y = x;
    let mut d: u128 = 1;

    let mut f = |x: u128| -> u128 { (x * x + 1) % n };

    while d == 1 {
        x = f(x);
        y = f(f(y));
        if x > y {
            d = (x - y).gcd(n);
        }
        else
        {
            d = (y - x).gcd(n);
        }
    }

    d
}


fn factorize_rho_method() -> () {
    println!("Введите число: ");
    let mut n = read_integer() as u128;
    println!("Введите эпсилон: ");
    read_float();
    let kekw = n;
    let mut factors = Vec::new();
    while n > 1 {
        let factor = rho_method(n);
        factors.push(factor);
        n /= factor;
    }

    println!("Разложение числа {}: {:?}", kekw, factors)
}

fn rho_minus_1_method(n: u64, mut base: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(2..n);

    let mut power = base;
    let mut d = 1;
    let upper_bound = 18446744073709551615 / 1000;
    while d == 1 {
        power *= 2;
        let a_powered = power_mod(a, power, n);
        d = (a_powered - 1).gcd(n);
        if power > upper_bound {
            if base == 2 {
                return n;
            }
            else {
                base = base - 1;
                power = base    
            }
        }
        if d != 1 {
            break;
        }
    }
    d
}


fn factorize_rho_minus_1_method() {
    println!("Введите число: ");
    let mut n = read_integer() as u64;
    println!("Введите базу B: ");
    let base = read_integer() as u64;
    let mut factors = Vec::new();
    let kekw = n;
    while n > 1 {
        let factor = rho_minus_1_method(n, base);
        factors.push(factor);
        n /= factor;
    }
    println!("Разложение числа {}: {:?}", kekw, factors)
}


fn continued_fraction_sqrt(n: u64) -> Vec<u64> {
    let mut a = (n as f64).sqrt() as u64;
    let mut m = 0;
    let mut d = 1;
    let mut num1 = a;
    let mut num2 = 1;
    let mut result = vec![a];

    while num1 * num1 != n {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a + m) / d;

        result.push(a);
        let num3 = num1;
        num1 = a * num1 + num2;
        num2 = num3;
    }
    println!("suka");

    result
}


fn eratho_sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            let mut multiple = i * i;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }
    let primes: Vec<usize> = (0..=n).filter(|&x| is_prime[x]).collect();
    primes
}

fn get_factor_base(n: u64) -> Vec<usize> {
    let m = f64::sqrt(((f64::exp(f64::sqrt(f64::ln(n as f64) * f64::ln(f64::ln(n as f64))))) as u64) as f64) as usize;
    let factor_base = eratho_sieve(m);
    factor_base
}

fn brillhart_morrison_method(n: u64) -> Vec<u64> {
    let factor_base = get_factor_base(n);
    println!("Фактор база: {:?}", factor_base);

    let mut smooth_b = Vec::new();
    for b in (f64::sqrt(n as f64) as u64)..n {
        let a = b.pow(2) % n;
        for &fact in &factor_base {
            let f = fact.pow(2) % n as usize;
            if a == f as u64 {
                smooth_b.push((b, fact));
            }
        }
    }
    let mut factors = HashSet::new();
    for (a, fact) in smooth_b {
        let factor = (a - fact as u64).gcd(n);
        if factor != 1 {
            factors.insert(factor);
        }
    }
    let mut result: Vec<u64> = factors.into_iter().collect();
    result.sort();
    result
}

// 7839991
// a = 
fn factorize_brillhart_morrison_method() {
    println!("Введите число: ");
    let mut n = read_integer() as u64;
    let mut factors = brillhart_morrison_method(n);
    let kekw = n;
    println!("Разложение числа {}: {:?}", kekw, factors)
}


fn main() {
    println!("Выберите метод разложения:");
    println!("1. p-метод Полларда");
    println!("2. (p-1)-метод Полларда");
    println!("3. Метод цепных дробей (Бриллхарта-Моррисона)");
    
    let chosen_method = read_integer();
    match chosen_method {
        1 => factorize_rho_method(),
        2 => factorize_rho_minus_1_method(),
        3 => factorize_brillhart_morrison_method(),
        _ => println!("Введено неверное число!"),
    }

}
