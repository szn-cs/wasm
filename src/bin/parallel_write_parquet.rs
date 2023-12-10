use num_cpus;
use std::env;
fn main() {
    {
        let args: Vec<String> = env::args().collect();

        let n: usize;
        let t: usize;

        if args.len() == 3 {
            n = args[1].clone().trim_end().parse::<usize>().unwrap(); // e.g. n = 50_000_000
            t = args[2].clone().trim_end().parse::<usize>().unwrap();
        } else {
            n = 50_000_000;
            t = num_cpus::get();
        }

        use wasm_assessment::parallel_read::parallel_write_parquet;
        let _ = parallel_write_parquet::run(n, t);
    }

    println!("done");
}
