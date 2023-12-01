
run() {
    cargo +nightly run --bin read_parallel
}

benchmark_env() {
    sbatch ./slurm.sh && watch -n 2  --differences=cumulative "squeue -u <user>" 
}