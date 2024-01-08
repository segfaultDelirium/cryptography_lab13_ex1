use rand::Rng; // 0.8.5

/*

[dependencies]
rand = "0.8.5"

 */

fn check_if_prime(n: u128){
    let isPrime = miller_rabin(n, n);
    println!("n = {n} is prime? {isPrime}");
}

fn main() {
    println!("algorytm millera rabina");

    check_if_prime(561);
    check_if_prime(1729);
    check_if_prime(6601);
    check_if_prime(881);
    check_if_prime(9677);
    check_if_prime(17321);
    check_if_prime(37579);
}


fn find_correct_t_u(n: u128) -> (u128, u128) {
    for t in 1..n{
        let u: f64 = (n as f64 - 1.0) / (2.0_f64.powf(t as f64));
        if (u - (u as i64) as f64) < 0.0005 {
            return (t, u as u128)
        }

    }
    (0, 0)
}

fn witness(a: u128, n: u128) -> bool{
    let (t, u) = find_correct_t_u(n);
    // println!("t = {t}, u = {u}");
    let x0 = modulo_pow(a, u, n);
    let mut xi_minus1 = x0;
    for i in 1..=t{
        let mut xi = modulo_pow(xi_minus1, 2, n);
        // xi == -1 &&
        if xi_minus1 != 1 && xi_minus1 != n -1 {
            return true;
        }
        xi_minus1 = xi;
    }
    if xi_minus1 != 1 {
        return true;
    }

    false
}

// jesli pierwsza to true, jesli zlozona to false
fn miller_rabin(n: u128, s: u128) -> bool {
    for j in 1..=s{
        let a = rand::thread_rng().gen_range(1..=n-1);
        if witness(a, n){
            return false;
        }
    }
    true
}

fn modulo_pow(a: u128, j: u128, n: u128) -> u128{
    let mut res = a;
    for _i in 1..j{
        res = modulo_euclid((res * a) as i128, n as i128) as u128;
    }

    res
    // a
}

fn modulo_euclid(j: i128, k: i128) -> i128 {
    let res =  j % k;
    if res < 0 {return res + k} else {return res}
}
