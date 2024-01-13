//! Aufgabe 2.1: Primzahltest

fn main() {
    for n in 1..21 {
        let star = if is_prime(n) {"*"} else {""};
        println!("{}{}", n, star);
    }
}

fn is_prime(n : u64) -> bool {
    if n < 2 { 
        return false; 
    }
    let mut divisor = 2;
    while divisor * divisor <= n {
        if (n % divisor) == 0 {
            return false;
        }
        divisor += 1;
    }
    return true;
}

#[test]
fn small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
}

#[test]
fn small_composites() {
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
}

#[test]
fn large_primes() {
    assert!(is_prime(1_300_769));
    assert!(is_prime(1_300_297));
    assert!(is_prime(7_367_287));
}

#[test]
fn large_composites() {
    assert!(!is_prime(908_209));
    assert!(!is_prime(3_073_009));
    assert!(!is_prime(4_897_369));
}
