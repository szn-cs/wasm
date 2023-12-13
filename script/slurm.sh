#!/usr/bin/env zsh

#SBATCH -p instruction
#SBATCH --job-name=task
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=1
#SBATCH --cpus-per-task=40
#SBATCH --time=00:30:00
#SBATCH --mem=20G

mkdir -p resource

cargo +nightly build -v --features parallel_read --release --bin parallel_write_parquet
cargo +nightly build -v --features script --release --bin scaling_analysis

./target/release/scaling_analysis
