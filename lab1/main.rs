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
    println!("Введите число a:");
    let a = read_integer();

    println!("Введите число b:");
    let b = read_integer();

    println!("Алгоритм Евклида: {}", euclid_gcd(a, b));
    println!("Расширенный алгоритм Евклида: {:?}",
            euclid_gcd_extended(a, b, 0, 0));
    println!("Бинарный алгоритм Евклида: {}", euclid_gcd_binary(a, b));    
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


fn chinese_remainder_theorem(u_values: Vec<i32>, modules: Vec<i32>, n: i32) -> i32 {
    let mut big_m: i32 = 1;
    for i in 0..n {
        big_m *= modules[i as usize];
    }

    let mut params: Vec<i32> = Vec::new();
    let mut bezouts: Vec<i32> = Vec::new();
    for i in 0..n {
        let cur_m: i32 = modules[i as usize];
        let cur_param: i32 = big_m / cur_m;
        params.push(cur_param);

        let (_, _, inv, _) = euclid_gcd_extended(cur_param, cur_m, 0, 0);
        bezouts.push(inv);
    }

    let mut solution: i32 = 0;
    for i in 0..n {
        solution += u_values[i as usize]
                    * bezouts[i as usize]
                    * params[i as usize];
    }

    println!("{}", solution.rem_euclid(big_m));
    return 0;
}


fn harner_algorithm(u_values: Vec<i32>, modules: Vec<i32>, n: i32) -> i32 {
    let mut coeffs: Vec<Vec<i32>> = Vec::new();

    for i in 0..n {
        let mut cur_cs: Vec<i32> = Vec::new();
        let m_i: i32 = modules[i as usize];
        for j in 0..n {
            let m_j: i32 = modules[j as usize];
            let (_, _, cur_c, _) = euclid_gcd_extended(m_i, m_j, 0, 0);
            cur_cs.push(cur_c.rem_euclid(m_j));
        }
        coeffs.push(cur_cs);
    }

    let mut qs: Vec<i32> = Vec::new();

    for i in 0..n {
        let mut cur_q: i32 = u_values[i as usize];
        for j in 0..qs.len() {
            cur_q = (cur_q - qs[j as usize]) * coeffs[j as usize][i as usize]; 
        }
        qs.push(cur_q.rem_euclid(modules[i as usize]));
    }

    let mut solution: i32 = qs[0];
    
    for i in 1..n {
        let mut mult: i32 = 1;
        for j in 0..i {
            mult *= modules[j as usize];
        }
        solution += qs[i as usize] * mult;
    }

    print!("{}", solution);
    return 0;
}


fn solve_comparison() -> () {
    println!("Введите число сравнений: ");
    let n = read_integer();
    

    println!("Введите значения u: ");
    let mut u_values: Vec<i32> = Vec::new();
    for _i in 0..n {
        let u = read_integer();
        u_values.push(u);
    }

    println!("Введите модули сравнений: ");
    let mut modules: Vec<i32> = Vec::new();
    for _i in 0..n {
        let m = read_integer();
        modules.push(m);
    }

    if !check_coprime(modules.clone(), n) {
        println!("Некоторые модули не взаимнопросты!");
        return ();
    }

    println!("Греко-китайская теорема: ");
    chinese_remainder_theorem(u_values.clone(), modules.clone(), n);
    println!("Алгоритм Гарнера: ");
    harner_algorithm(u_values.clone(), modules.clone(), n);
    return ();
}


fn check_if_trivial(coeffs: Vec<i32>, b: i32) -> i32 {
    let mut is_cf_zeros: bool = true;
    for i in 0..coeffs.len() {
        if coeffs[i as usize] != 0 {
            is_cf_zeros = false;
            break;
        }
    }

    if is_cf_zeros && b == 0 {
        return 1;
    }

    if is_cf_zeros && b != 0 {
        return -1;
    }

    return 0;
}


fn mod_inverse(a: i32, n: i32) -> i32 {
    let (_, g, x, _) = euclid_gcd_extended(a, n, 0, 0);
    if g != 1 {
        return -1;
    } else {
        let result = (x % n + n) % n;
        return result;
    }
}

fn gauss () -> () {
    println!("Введите число уравнений: ");
    let n = read_integer();
    
    println!("Введите число неизвестных: ");
    let m = read_integer();

    println!("Введите модуль: ");
    let p = read_integer();

    println!("Введите коэффициенты: ");
    let mut a_values: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        println!("Введите коэффициенты {}-го уравнения:", i + 1);
        let mut a_line: Vec<i32> = Vec::new();
        for _j in 0..m {
            let a = read_integer();
            a_line.push(a);   
        }
        a_values.push(a_line);
    }

    println!("Введите свободные коэффициенты:");
    let mut terms: Vec<i32> = Vec::new();
    for _j in 0..n {
        let t = read_integer();
        terms.push(t);
    }

    let mut x: Vec<i32> = vec![0; m as usize];

    for i in 0..n {
        let is_trivial: i32 = check_if_trivial(a_values[i as usize].clone(),
                                               terms[i as usize].clone());
        if is_trivial == -1 {
            println!("Система не имеет решений!");
            return ();
        }

        let inv: i32 = mod_inverse(a_values[i as usize][i as usize], p);

        for j in i..m {
            a_values[i as usize][j as usize] = (a_values[i as usize][j as usize] * inv).rem_euclid(p);
        }

        terms[i as usize] = (terms[i as usize] * inv).rem_euclid(p);

        for k in (i + 1)..n {
            let fact: i32 = a_values[k as usize][i as usize];
            terms[k as usize] = (terms[k as usize] - terms[i as usize]  * fact).rem_euclid(p);
            if terms[k as usize] < 0 {
                terms[k as usize] += p;
            }

            for j in 0..m {
                a_values[k as usize][j as usize] = (a_values[k as usize][j as usize] - a_values[i as usize][j as usize] * fact).rem_euclid(p);

                if a_values[k as usize][j as usize] < 0 {
                    a_values[k as usize][j as usize] += p;
                }
            }
        }
    }

    for j in (0..terms.len()).rev() {
        x[j as usize] = terms[j as usize];
        for k in (j + 1)..m as usize{
            x[j as usize] = (x[j as usize] - a_values[j as usize][k as usize] * x[k as usize]).rem_euclid(p);
            if x[j as usize] < 0 {
                x[j as usize] += p;
            }
        }

        let inv: i32 = mod_inverse(a_values[j as usize][j as usize], p);
        x[j as usize] = (x[j as usize] * inv).rem_euclid(p);
    }

    println!("Методом Гаусса получена разряженная матрица:");
    for i in 0..n {
        let cur_line = &mut a_values[i as usize];
        cur_line.push(terms[i as usize]);
        print_array(cur_line.clone());
    }
    println!("Частное решение системы:");
    print_array(x.clone());
    println!("Общее решение системы:");
    for i in 0..m {
        if x[i as usize] == 0 {
            println!("x_{} = {}", i + 1, 0)
        }
        else {
            let mut ans = String::from("");
            for j in 0..m {
                let cur_a = a_values[i as usize][j as usize];
                if i != j {
                    if cur_a > 0 {
                        ans.push_str(" - ");
                        ans.push_str(&cur_a.to_string());
                    }
                    if cur_a < 0 {
                        ans.push_str(" + ");
                        ans.push_str(&(-cur_a).to_string());
                    }
                    if cur_a != 0
                    {
                        ans.push('x');
                        ans.push_str(&(j + 1).to_string());
                    }
                }
            }
            println!("x_{} = ({}{}) / {}", i + 1, terms[i as usize], ans, a_values[i as usize][i as usize])
        }
    }
    return ();
}


fn main() {
    println!("Выберите опцию:");
    println!("1 - алгоритмы Евклида;");    
    println!("2 - Греко-китайская теоремаа и алгоритм Гарнера;");    
    println!("3 - алгоритм Гаусса;");

    let n = read_integer();

    match n {
        1 => solve_euclid(),
        2 => solve_comparison(),
        3 => gauss(),
        _ => println!("Введено неверное число!"),
    }
}
