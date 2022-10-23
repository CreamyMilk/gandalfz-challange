/// egcd finds the greatest common divisor of two integers `a` and `b`,
/// and two integers `coefficient_a` and `coefficient_b`
/// such that` ( coefficient_a · x) + ( coefficient_b · y )` is the greatest common divisor of `coefficient_a` and `coefficient_b · x`
/// using the extended Euclidean algorithm.
fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a != 0 {
        let computed_mod = b % a;

        let (computed_gcd, coefficient_a, coefficient_b) = egcd(computed_mod, a);

        let quotient = b / a;

        let new_coefficient_a = coefficient_b - (quotient * coefficient_a);

        let new_coefficient_b = coefficient_a;

        return (computed_gcd, new_coefficient_a, new_coefficient_b);
    }

    return (b, 0, 1);
}

/// modular_mul_inverse calculates the [Modular Multiplicative Inverse]
/// of an integer `a` such that `a·x ≡ 1 (mod m)`
/// In the event this function will return `None`.
/// Otherwise, the inverse will be returned wrapped up in a `Some`.
fn modular_mul_inverse(a: isize, m: isize) -> Option<isize> {
    let (gcd, coefficient_a, _) = egcd(a, m);

    if gcd != 1 {
        return None;
    }

    return Some((coefficient_a % m + m) % m);
}

// we import the std::io create in order to read from standard-input
use std::io;

fn main() {
    println!("Welcome 👋 Input the two values you wish to compute the Modular Multiplicative Inverse for \n");
    println!("eg. 42 2017 \n");

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let inputs: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let a = inputs[0];
        let m = inputs[1];

        let result = modular_mul_inverse(a, m);
        match result {
            Some(computed_inv) => {
                println!("{a} and {m} have a Modular Multiplicative Inverse of -> {computed_inv} \n");
            }
            None => println!("🚨 Modular_Multiplicative_Inverse can't be computed for {a} mod {m}\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modular_mul_inverse;

    #[test]
    fn it_works() {
        assert!(
            modular_mul_inverse(15, 5).is_none(),
            "value for a if greater that the mod"
        );
        assert_eq!(modular_mul_inverse(13, 5).unwrap(), 2);
        assert_eq!(modular_mul_inverse(43, 600).unwrap(), 307);
        assert_eq!(modular_mul_inverse(42, 2017).unwrap(), 1969);
        assert_eq!(modular_mul_inverse(345, 76408).unwrap(), 48281);
    }
}
