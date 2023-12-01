#!/usr/bin/env zsh
#SBATCH -p instruction
#SBATCH --job-name=task
#SBATCH --nodes=2
#SBATCH --ntasks-per-node=1
#SBATCH --cpus-per-task=20
#SBATCH --time=00:10:00

# srun -n 2 --cpu-bind=none ./target/task

./target/scaling-analysis

