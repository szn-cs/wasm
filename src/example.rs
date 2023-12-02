#[cfg(feature = "example")]

pub fn threads() {
    use rayon::prelude::*;
    use std::time;

    let mut data = vec![0; 1_000_000];

    let now = time::Instant::now();
    {
        data.par_iter_mut().for_each(|item| {
            *item += 1;
        });
    }
    let elapsed = now.elapsed();
    let duration_ms = elapsed.as_nanos() as f64 / 1_000_000 as f64; // real time execution duration

    println!("{}", duration_ms);
}
