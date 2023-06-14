#!/usr/bin/env bash

# this script takes input files as separate arguments and runs the solver on each of them
# can be invoked as
# `./scripts/run_multple.sh ../instances/somwhere/else/*`

cargo build --release

printf "running %s instances\n" "$#"

echo 'slot,optimal attesters,optimal score,greedy attesters,greedy score,gap,MIP time (ms),total time (ms),min cliques by AD,max cliques by AD,avg. cliques by AD,attesters,unique data_roots' >> report.csv;
for instance in "$@"; do
  ./target/release/sigmaprime --approach mip_approach --input $instance >> report.csv; 
done
