use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();

        let (file_path, t): (&str, usize) = if args.len() == 3 {
            (
                &args[1],
                args[2].clone().trim_end().parse::<usize>().unwrap(),
            )
        } else {
            ("./resource/House_Price.parquet", num_cpus::get())
        };

        use wasm_assessment::parallel_read;
        let _ = parallel_read::run_multithread(file_path, t).unwrap();
    }
}
