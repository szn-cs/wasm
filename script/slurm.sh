#!/usr/bin/env zsh

#SBATCH -p instruction
#SBATCH --job-name=task
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=1
#SBATCH --cpus-per-task=40
#SBATCH --time=00:10:00

mkdir -p resource

cargo +nightly build -v --features parallel_read --release

./target/release/scaling-analysis
