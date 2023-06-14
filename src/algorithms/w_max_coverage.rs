#![allow(unused_imports, dead_code)]

use good_lp::variable::UnsolvedProblem;
use good_lp::{
    constraint, default_solver, variable, variables, Expression, IntoAffineExpression, Solution,
    SolverModel,
};
use std::iter::Sum;
use std::ops::Mul;

/// A problem instance for the Weighted Maximum Coverage problem with sets `sets`, element weights
/// `weights`, and maximum limit `k`.
pub struct WeightedMaximumCoverage {
    /// Sets to select from
    pub sets: Vec<Vec<usize>>,
    /// Weights of the elements
    pub weights: Vec<f64>,
    /// Limit for the size of the solution (number of selected sets)
    pub k: usize,
}

#[allow(unused_variables)]
impl WeightedMaximumCoverage {
    /// Solves the Weighted Maximum Coverage using a MIP solver.
    pub fn solve(&self) -> Result<Vec<usize>, &str> {
        // produce lists of sets containing a given element
        let mut sets_with: Vec<Vec<usize>> = vec![];
        sets_with.resize_with(self.weights.len(), Vec::new);
        for i in 0..self.sets.len() {
            for &j in &self.sets[i] {
                sets_with[j].push(i);
            }
        }

        let mut vars = variables!();

        // initialise set variables
        let xs = vars.add_vector(variable().binary(), self.sets.len());

        // initialise element variables
        let ys = vars.add_vector(variable().min(0.0).max(1.0), self.weights.len());

        // define objective function as linear combination of element variables and weights
        let objective =
            Expression::sum((0..self.weights.len()).map(|yi| ys[yi].mul(self.weights[yi])));
        let mut problem = vars.maximise(objective).using(default_solver);

        // limit solution size to k sets
        problem = problem.with(Expression::sum(xs.iter()).leq(self.k as f64));

        // add constraint allowing to cover an element only if one of the sets containing it is included
        for j in 0..self.weights.len() {
            problem = problem.with(constraint! {
                Expression::sum(sets_with[j].iter().map(|i| xs[*i])) >= ys[j]
            });
        }

        // tell CBC not to log
        problem.set_parameter("log", "0");

        // should be safe to `unwrap` since the problem is underconstrained
        let solution = problem.solve().unwrap();

        // report solution
        let mut coverage = Vec::with_capacity(self.weights.len());
        xs.iter()
            .enumerate()
            .filter(|(_, &x)| solution.value(x) > 0.0)
            .for_each(|(i, _)| coverage.push(i));

        Ok(coverage)
    }
}

mod tests {
    use crate::algorithms::w_max_coverage::WeightedMaximumCoverage;

    #[test]
    fn small_coverage() {
        let p = WeightedMaximumCoverage {
            sets: vec![
                vec![0, 1, 2],
                vec![0, 3],
                vec![1, 2],
                vec![3, 2],
                vec![0, 4],
                vec![2, 3, 0],
            ],
            weights: vec![12.1, 11.3, 3.9, 2.3, 8.2],
            k: 2,
        };

        println!("{:?}", p.solve());
    }
}
