use ndarray::Array;
use ndarray_rand::{rand_distr::Uniform, RandomExt};

/// Birthday problem
fn main() {
    let k: usize = 23;
    let n: usize = 365;

    println!("Probability of same birthday: {}", calculate(k, n));
    println!("Simulated probability of same birthday: {}", simulate(k, n));
}

fn calculate(k: usize, n: usize) -> f64 {
    let mut all_different_birthday_outcomes = 1.;
    for i in (n - k + 1)..=n {
        all_different_birthday_outcomes *= i as f64;
    }

    let sample_space_size = (n as f64).powf(k as f64);

    let prob_any_same_birthday = 1. - all_different_birthday_outcomes / sample_space_size;

    prob_any_same_birthday
}

fn simulate(k: usize, n: usize) -> f64 {
    let iterations = 10_000;
    let mut any_same_birthday_counts = 0;

    for _ in 0..iterations {
        let birthdays = Array::random(k, Uniform::new_inclusive(1, n));

        // check if any two birthdays are the same
        let mut any_same_birthday = false;
        'comp: for i in 0..k {
            for j in (i + 1)..k {
                if birthdays[i] == birthdays[j] {
                    any_same_birthday = true;
                    break 'comp;
                }
            }
        }

        if any_same_birthday {
            any_same_birthday_counts += 1;
        }
    }

    any_same_birthday_counts as f64 / iterations as f64
}
