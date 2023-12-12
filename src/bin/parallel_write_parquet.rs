use num_cpus;
use std::env;
use wasm_assessment::parallel_read::parallel_write_parquet;

fn main() {
    {
        let args: Vec<String> = env::args().collect();

        let n: usize;
        let t: usize;

        if args.len() == 3 {
            n = args[1].clone().trim_end().parse::<usize>().unwrap(); // e.g. n = 50_000_000
            t = args[2].clone().trim_end().parse::<usize>().unwrap();
        } else {
            n = 10_000_000;
            t = num_cpus::get();
        }

        let maximum_vthreads = 40; // Euler: logical cores = 2 sockets * 10 cores per socket * 2  Threads per core
        let num_columns = maximum_vthreads; // aim for maximum number of vThreds (# cores * 2)

        let _ = parallel_write_parquet::run(num_columns, n, t);
    }
}
