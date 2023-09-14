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


fn print_array (arr: Vec<i32>) {
    print!("Array: [");
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


fn chinese_remainder_theorem () -> i32 {
    println!("Print comparison amount: ");
    let n = read_integer();
    

    println!("Print u values: ");
    let mut u_values: Vec<i32> = Vec::new();
    for i in 0..n {
        let u = read_integer();
        u_values.push(u);
    }

    println!("Print modules: ");
    let mut modules: Vec<i32> = Vec::new();
    for i in 0..n {
        let m = read_integer();
        modules.push(m);
    }

    // check if modules is coprime
    if !!!check_coprime(modules.clone(), n) {
        println!("Some modules aren't coprime!");
        return 1;
    }

    let mut big_m: i32 = 1;
    for i in 0..n {
        big_m *= modules[i as usize];
    }

    let mut params: Vec<i32> = Vec::new();
    let mut bezouts: Vec<i32> = Vec::new();
    for i in 0..n {
        let mut cur_m: i32 = modules[i as usize];
        let mut cur_param: i32 = big_m / cur_m;
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


fn harner_algorithm () -> i32 {
    println!("Print comparison amount: ");
    let n = read_integer();
    
    println!("Print u values: ");
    let mut u_values: Vec<i32> = Vec::new();
    for i in 0..n {
        let u = read_integer();
        u_values.push(u);
    }

    println!("Print modules: ");
    let mut modules: Vec<i32> = Vec::new();
    for i in 0..n {
        let m = read_integer();
        modules.push(m);
    }

    // check if modules is coprime
    if !!!check_coprime(modules.clone(), n) {
        println!("Some modules aren't coprime!");
        return 1;
    }

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


fn gauss () -> i32 {
    println!("Print equation amount: ");
    let n = read_integer();
    
    println!("Print coefficients: ");
    let mut a_values: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        println!("Print coeffs for first equation:");
        let mut a_line: Vec<i32> = Vec::new();
        for j in 0..n {
            let a = read_integer();
            a_line.push(a);   
        }
        a_values.push(a_line);
    }

    println!("Print constant terms:");
    let mut terms: Vec<i32> = Vec::new();
    for j in 0..n {
        let t = read_integer();
        terms.push(t);
    }

    

    return 0;
}


fn main() {
    // solve_euclid();
    // chinese_remainder_theorem();
    // harner_algorithm();
    gauss();
}
