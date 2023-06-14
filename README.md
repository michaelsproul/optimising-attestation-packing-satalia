This is an archive of code and data used in the joint Satalia <> Sigma Prime project on
attestation packing.

For up-to-date versions of the reports, prefer those in the blog post:

https://lighthouse-blog.sigmaprime.io/optimising-attestation-packing.html

This repository exists primarily as a reference for the open source libraries produced by Satalia
for Bron-Kerbosch and max coverage using linear programming.

---

# Attestation Packing Problem

This repository contains the formal description of the Attestation Aggregation
and Packing Problem (AAPP) as well as the code for several approaches to solve
it optimally and heuristically, and some test instances.

## Dependencies

This crates uses [good_lp](https://crates.io/crates/good_lp) for modelling the
[Weighted Maximum Coverage
Problem](https://en.wikipedia.org/wiki/Maximum_coverage_problem#Weighted_version),
as a MIP problem, which the approaches implemented here rely on. While
**good_lp** is sufficient for modelling, solving the model requires a working
MIP solver. **good_lp**'s default solver is
[CBC](https://github.com/coin-or/Cbc) which can be easily installed, e.g., on
Ubuntu

``` 
sudo apt-get install coinor-cbc coinor-libcbc-dev 
```

will satisfy all the dependencies of the project.

## Building & running

This is a Cargo project, so the usual workflow applies. Build using

```
cargo build --release
```

run (e.g., using an instance provided with the repository) using

```
cargo run --release -- --input instances/slot_3741052_instance.json
```

## Reporting

The exact solver can be run an all instances in a directory and produce a CSV
file (`report.csv`) using the `run_mip.sh` or `run_mip_zstd.sh` scripts in the
`scripts/` directory. Precise usage instructions are given in those scripts.

## Documents

* [Problem formulation with background](doc/problem.pdf)
* [Problem formulation](doc/abridged.pdf)
* [Approaches (MIP, decomposed)](doc/approaches.pdf)

This is a Satalia project for [Sigma Prime](https://sigmaprime.io/).
