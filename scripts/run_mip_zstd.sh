#!/usr/bin/env bash

# this script takes a directory that contains input files and runs the solver on all of them
# `./scripts/run_mip_zstd.sh ../instances/somwhere/else`

cargo build --release

printf "running %s instances\n" `find $1 -maxdepth 1 -type f | wc -l`

echo 'slot,optimal attesters,optimal score,greedy attesters,greedy score,gap,MIP time (ms),total time (ms),min cliques by AD,max cliques by AD,avg. cliques by AD,attesters,unique data_roots' >> report.csv;
for instance in `find $1 -maxdepth 1 -type f`; do
  zstd -dq $instance -o instance.json
  RUST_BACKTRACE=1 ./target/release/sigmaprime --approach mip_approach --input instance.json >> report.csv; 
  rm instance.json
done
