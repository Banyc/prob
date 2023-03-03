use ndarray::{Array, Axis};
use ndarray_rand::{RandomExt, SamplingStrategy};

/// Matching problem simulation
fn main() {
    let n = 100;
    let objects = Array::range(0.0, n as f64, 1.0);
    let mut sum_of_any_matches = 0;
    let iterations = 10_000;
    for _ in 0..iterations {
        let sample = objects.sample_axis(Axis(0), n, SamplingStrategy::WithoutReplacement);
        // println!("{:?}", sample);

        // element-wise comparison
        let matches = objects
            .iter()
            .zip(sample.iter())
            .map(|(a, b)| a == b)
            .collect::<Vec<_>>();
        let matches = Array::from(matches).mapv(|x| if x { 1 } else { 0 });
        // println!("{:?}", matches);

        if matches.sum() > 0 {
            sum_of_any_matches += 1;
        }
    }

    println!(
        "Simulated probability of any match: {}",
        sum_of_any_matches as f64 / iterations as f64
    );
}
