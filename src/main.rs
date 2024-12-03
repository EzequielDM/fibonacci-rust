use num::{BigUint, Zero, One};

fn main() {
    // Parse input to u32
    let n: u32 = 1000000;

    let fib_n = fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, fib_n);
}

fn fibonacci(n: u32) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }
    if n == 1 {
        return BigUint::one();
    }

    let (fib_n, _) = fast_doubling(n);
    fib_n
}

fn fast_doubling(n: u32) -> (BigUint, BigUint) {
    if n == 0 {
        return (BigUint::zero(), BigUint::one());
    }

    // Recursive call: compute F(k) and F(k+1) where k = n / 2
    let (a, b) = fast_doubling(n / 2);

    // Compute F(2k) = F(k) * [2*F(k+1) - F(k)]
    let two = BigUint::from(2u32);
    let c = &a * (&b * &two - &a);

    // Compute F(2k + 1) = F(k)^2 + F(k+1)^2
    let d = &a * &a + &b * &b;

    if n % 2 == 0 {
        (c, d)
    } else {
        (d.clone(), &c + &d)
    }
}
