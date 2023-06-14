#!/usr/bin/env bash

# this script takes input files as separate arguments and runs the solver on each of them
# can be invoked as
# `./scripts/run_multple.sh ../instances/somwhere/else/*`

cargo build --release

printf "running %s instances\n" "$#"

echo 'n_unique_roots' >> roots.csv;
for instance in "$@"; do
  ./target/release/sigmaprime --approach n_unique_roots --input $instance >> roots.csv; 
done
