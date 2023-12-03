use std::io;
use rand::Rng;
// use quadratic::jacobi;
use rand::distributions::Uniform;

fn read_integer() -> i128 {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i128 = n.trim().parse().expect("invalid input");
    n
}


fn euclid_gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    };
    let r = a % b;
    return euclid_gcd(b, r);
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


fn check_coprime(modules: Vec<i128>, n: i128) -> bool {
    for i in 0..n {
        for j in 0..n {
            let m_i = modules[i as usize];
            let m_j = modules[j as usize];
            if euclid_gcd(m_i, m_j) != 1 && i != j {
                return false;
            }
        }
    }
    return true;
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


fn power_mod(base: i128, exponent: i128, modulus: i128) -> i128 {
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


fn fermat_test(number: i128) -> i32 {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=number - 1);
    
    if euclid_gcd(a, number) == 1 {
        let powered_a = power_mod(a, number - 1, number);
        if powered_a == 1 {
            // println!("Число {}, вероятно, простое.", number);
            return 1;
        }
        else {
            // println!("Число {} составное.", number);
            return 0;
        }
    }
    else {
        // println!("Число {} составное.", number);
        return 0;
    }
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


fn solovei_strassen_test(number: i128) -> i32 {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=number - 1);
    
    if euclid_gcd(a, number) == 1 {
        let powered_a = power_mod(a, (number - 1) / 2, number);
        let jacobi_a_n = jacobi_symbol(a, number);
        if powered_a == jacobi_a_n.rem_euclid(number) {
            // println!("Число {}, вероятно, простое.", number);
            return 1;
        }
        else {
            // println!("Число {} составное.", number);
            return 0;
        }
    }
    else {
        // println!("Число {} составное.", number);
        return 0;
    }
}


fn miller_rabin_test(number: i128) -> i32 {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=number - 1);
    
    if euclid_gcd(a, number) == 1 {
        let mut t = number - 1;
        let mut s = 0;
        while t % 2 == 0 {
            t = t / 2;
            s = s + 1;
        }

        let r_0 = power_mod(a, t, number);
        if r_0 != 1 && r_0 != (number - 1) {
            // println!("Число {} составное.", number);
            return 0;
        }

        for k in 1..(s - 1) {
            let r_k = power_mod(a, 2_i128.pow(k) * t, number);
            if r_k != (number - 1) {
                // println!("Число {} составное.", number);
                return 0;
            }
        }
        
        return 1;
        // println!("Число {}, вероятно, простое.", number);
    }
    else {
        return 0;
        // println!("Число {} составное.", number);
    }
}


fn main() {
    println!("Введите число:");
    let number = read_integer();
    println!("");
    
    println!("Введите количество раундов:");
    let rounds = read_integer();
    println!("");

    let mut is_prime_fermat = 0;
    let mut is_prime_ss = 0;
    let mut is_prime_mr = 0;

    let mut went_rounds = 0;
    while went_rounds != rounds {
        is_prime_fermat = is_prime_fermat + fermat_test(number);
        is_prime_ss = is_prime_ss + solovei_strassen_test(number);
        is_prime_mr = is_prime_mr + miller_rabin_test(number);
        went_rounds = went_rounds + 1;
    }

    println!("Тест Ферма:");
    if (i128::from(is_prime_fermat) > rounds / 2) {
        println!("Число {}, вероятно, простое.", number);
    }
    else {
        println!("Число {} составное.", number);
    }
    // fermat_test(number);
    println!("");

    println!("Тест Соловея-Штрассена:");
    // solovei_strassen_test(number);
    if (i128::from(is_prime_ss) > rounds / 2) {
        println!("Число {}, вероятно, простое.", number);
    }
    else {
        println!("Число {} составное.", number);
    }
    println!("");

    println!("Тест Миллера-Рабина:");
    // miller_rabin_test(number);
    if (i128::from(is_prime_mr) > rounds / 2) {
        println!("Число {}, вероятно, простое.", number);
    }
    else {
        println!("Число {} составное.", number);
    }
    println!("");

}
