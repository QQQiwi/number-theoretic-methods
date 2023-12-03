use std::io;
use rand::Rng;
// use quadratic::jacobi;
use rand::distributions::Uniform;

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
        let division = b / a;
        let x = y1 - division * x1;
        let y = x1;
        return (0, d, x, y)
    }
}


fn continued_fraction(mut a: i32, mut b: i32) -> Vec<i32> {
    let mut fraction: Vec<i32> = Vec::new();
    while b != 0 {
        fraction.push(a / b);
        let c = a;
        a = b;
        b = c % b;
    }
    fraction
}


fn get_p_n_q(fraction: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut p: Vec<i32> = vec![0, 1];
    let mut q: Vec<i32> = vec![1, 0];
    for i in 2..fraction.len() + 2 {
        let coef = fraction[i - 2 as usize];
        p.push(p[(i - 1) as usize] * coef + p[(i - 2) as usize]);
        q.push(q[(i - 1) as usize] * coef + q[(i - 2) as usize]);
    }
    return (p, q)
}


fn print_array(arr: Vec<i32>) {
    print!("[");
    for i in 0..arr.len() {
        print!("{}", arr[i]);
        if i < arr.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}


fn check_coprime(modules: Vec<i32>, n: i32) -> bool {
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


fn mod_inverse(a: i32, n: i32) -> i32 {
    let fraction: Vec<i32> = Vec::new();
    let (_, g, x, _) = euclid_gcd_extended(a, n, 0, 0);
    if g != 1 {
        return -1;
    } else {
        let result = (x % n + n) % n;
        return result;
    }
}


fn is_prime(n: i32) -> bool {
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


fn solve_continued_fraction() -> () {
    println!("Введите рациональное число r = a_0/a_1.");
    println!("Введите a_0:");
    let a0 = read_integer();
    println!("Введите a_1:");
    let a1 = read_integer();

    let fraction: Vec<i32> = Vec::new();
    let mut fraction = continued_fraction(a0, a1);
    println!("Цепная дробь:");
    print_array(fraction);
}


fn diophantine_solution(a: i32, b: i32, c: i32) -> (i32, i32, i32, i32) {
    let fraction: Vec<i32> = Vec::new();
    let mut fraction = continued_fraction(a, b);

    let p: Vec<i32> = Vec::new();
    let q: Vec<i32> = Vec::new();
    let (p, q) = get_p_n_q(fraction.clone());
    
    let k = fraction.len();

    let x = q[k as usize] * i32::pow(-1, k as u32) * c;
    let y = p[k as usize] * i32::pow(-1, k as u32) * c;

    (x, y, a, b)
}


fn solve_diophantine() -> () {
    println!("Введите a:");
    let a = read_integer();
    println!("Введите b:");
    let b = read_integer();
    println!("Введите c:");
    let c = read_integer();

    let (x, y, a, b) = diophantine_solution(a, b, c);

    println!("Решениями уравнения являются:");
    println!("x = {} + {}t", x, b);
    println!("y = {} + {}t", y, a);
}


fn linear_comparison_solution() -> () {
    println!("Введите a:");
    let a = read_integer();
    println!("Введите b:");
    let b = read_integer();
    println!("Введите m:");
    let m = read_integer();

    let (_, d, p, q) = euclid_gcd_extended(a, m, 0, 0);

    if (b % d) == 0 {
        println!("Сравнение имеет {} решений.", d);
    }
    else {
        println!("Сравнение не имеет решений.");
        return ();
    }

    let new_b = b / d;
    let new_m = m / d;

    let mut k = 1;
    if (d - 1) % 2 != 0 {
        k = -1;
    }

    let first_ans = ((k * p * new_b) % new_m).rem_euclid(m);
    let mut solutions: Vec<i32> = vec![first_ans];

    for i in 1..d {
        solutions.push((solutions[0 as usize] + new_m * i).rem_euclid(m));
    }

    print_array(solutions.clone());
}


fn power_mod(base: i32, exponent: i32, modulus: i32) -> i32 {
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


fn get_inverse_elem() -> () {
    println!("Введите элемент a, для которого хотите найти обратный:");
    let a = read_integer();
    println!("Введите модуль m:");
    let m = read_integer();
    let b = 1;

    let (_, d, p, q) = euclid_gcd_extended(a, m, 0, 0);
    let new_b = b / d;
    let new_m = m / d;
    let mut k = 1;
    if (d - 1) % 2 != 0 {
        k = -1;
    }
    let first_ans = ((k * p * new_b) % new_m).rem_euclid(m);

    println!("Обратный элемент: {}", first_ans);    
}


fn continued_fraction_application() -> () {
    println!("Выберите приложение:");
    println!("1 - решение линейных диофантовых уравнений;");    
    println!("2 - вычисление обратных элементов в кольце вычетов Z_m;");    
    println!("3 - решение линейных сравнений ax = b (mod m);");

    let n = read_integer();

    match n {
        1 => solve_diophantine(),
        2 => get_inverse_elem(),
        3 => linear_comparison_solution(),
        _ => println!("Введено неверное число!"),
    }
}


fn legendre_symbol(a: i32, mut p: i32) -> i32 {
    if is_prime(p) {
        return jacobi_symbol(a, p);
    }

    let mut a = a % p;
    if a < 0 {
        a += p;
    }

    let mut result = 1;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            if p % 8 == 3 || p % 8 == 5 {
                result = -result;
            }
        }

        std::mem::swap(&mut a, &mut p);

        if a % 4 == 3 && p % 4 == 3 {
            result = -result;
        }

        a %= p;
    }

    if p == 1 {
        result
    } else {
        0
    }
}


fn jacobi_symbol(mut a: i32, mut n: i32) -> i32 {
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


fn eval_quadratic_residues() -> () {
    println!("Введите a:");
    let a = read_integer();
    println!("Введите p:");
    let p = read_integer();

    let (r1, r2) = find_square_roots(a.rem_euclid(p), p);
    if r1 == r2 && r1 == 0 {
        println!("Для заданных a и p квадратных корней найти не удалось.");
        return ();
    }
    if r1 != 0 {
        println!("Найденный корень: {}", r1);
    }
    if r2 != 0 {
        println!("Найденный корень: {}", r2);
    }
}


fn sqrt_mod(a: i32, p: i32) -> i32 {
    if legendre_symbol(a, p) != 1 {
        return -1;
    }

    let mut q = p - 1;
    let mut s = 0;

    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    if s == 1 {
        let x = a.pow(((p + 1) / 4) as u32) % p;
        return x;
    }

    let mut z = 2;

    while legendre_symbol(z, p) != -1 {
        z += 1;
    }

    let mut c = z.pow(q as u32) % p;
    let mut r = a.pow(((q + 1) / 2) as u32) % p;
    let mut t = a.pow(q as u32) % p;
    let mut m = s;

    while t != 1 {
        let mut i = 0;
        let mut zz = t;

        while zz != 1 {
            zz = (zz * zz) % p;
            i += 1;
        }

        let b = c.pow(2u32.pow((m - i - 1) as u32) as u32) % p;
        r = (r * b) % p;
        t = (t * b.pow(2)) % p;
        c = b.pow(2) % p;
        m = i;
    }

    r
}


fn find_square_roots(a: i32, p: i32) -> (i32, i32) {
    let mut root1 = 0;
    let mut root2 = 0;

    for i in 0..p {
        if (i*i % p) == a {
            if root1 == 0 {
                root1 = i;
            } else {
                root2 = i;
                break;
            }
        }
    }

    (root1, root2)
}


fn eval_symbols() -> () {
    println!("Введите a:");
    let a = read_integer();
    println!("Введите p:");
    let p = read_integer();

    println!("Символ Лежандра: {}", legendre_symbol(a, p));
    println!("Символ Якоби: {}", jacobi_symbol(a, p));
}


fn main() {
    println!("Выберите опцию:");
    println!("1 - разложение числа в цепную дробь;");    
    println!("2 - алгоритм приложения цепных дробей;");    
    println!("3 - алгоритмы вычисления символов Лежандра и Якоби;");
    println!("4 - алгоритм извлечения квадратного корня в кольце вычетов;");

    let n = read_integer();

    match n {
        1 => solve_continued_fraction(),
        2 => continued_fraction_application(),
        3 => eval_symbols(),
        4 => eval_quadratic_residues(),
        _ => println!("Введено неверное число!"),
    }
}
