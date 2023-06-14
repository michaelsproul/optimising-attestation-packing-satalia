use crate::aapp::instance::{Attestation, AttestationData, EpochAttesterID, Instance};
use crate::algorithms::{bron_kerbosh, WeightedMaximumCoverage};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub fn mip_approach(instance: &Instance) {
    let start = Instant::now();

    // group attestations by data
    let aggregated_atts = group_by_att_data(&instance.aggregated_attestations);
    let unaggregated_atts = group_by_att_data(&instance.unaggregated_attestations);

    // attester sets for cliques across attestation data
    let mut cliqued_atts: Vec<Vec<EpochAttesterID>> = vec![];

    let mut num_cliques = vec![];

    for (data, attestations) in &aggregated_atts {
        // derive cliques for current attestation data
        let cliques = bron_kerbosh(attestations, is_compatible);
        let mut cliques: Vec<Vec<EpochAttesterID>> = cliques
            .iter()
            .map(|clique| {
                clique
                    .iter()
                    .flat_map(|att_idx| &attestations[*att_idx].attesting_indices)
                    .cloned()
                    .collect()
            })
            .collect();

        // extract attester sets
        let attesters: Vec<EpochAttesterID> = unaggregated_atts
            .get(data)
            .cloned()
            .or_else(|| Some(vec![]))
            .unwrap()
            .iter()
            .map(|attestation| attestation.attesting_indices[0])
            .collect();

        for attester in &attesters {
            for clique in &mut cliques {
                if !clique.contains(attester) {
                    clique.push(*attester)
                }
            }
        }

        // // add aggregated attestation based on all unaggregated attestations (U)
        // cliques.push(attesters);

        num_cliques.push(cliques.len());

        cliqued_atts.extend(cliques);
    }

    // include aggregated attestations from unaggregated attestations whose attestation data doesn't come with aggregated attestations
    for (data, attestations) in &unaggregated_atts {
        if !aggregated_atts.contains_key(data) {
            let attesters: Vec<EpochAttesterID> = attestations
                .iter()
                .map(|attestation| attestation.attesting_indices[0])
                .collect();

            cliqued_atts.push(attesters);
        }
        num_cliques.push(1);
    }

    let mut new_to_old: Vec<EpochAttesterID> = cliqued_atts.iter().flatten().cloned().collect();
    new_to_old.sort_unstable();
    new_to_old.dedup();

    let old_to_new: HashMap<EpochAttesterID, usize> = new_to_old
        .iter()
        .enumerate()
        .map(|(idx, attester)| (*attester, idx))
        .collect();

    let reindexed_atts: Vec<Vec<usize>> = cliqued_atts
        .iter()
        .map(|clique| clique.iter().map(|attester| old_to_new[attester]).collect())
        .collect();

    let reindexed_weights: Vec<f64> = new_to_old
        .iter()
        .map(|attester| instance.reward_function.get(attester))
        .map(|weight| weight.copied().unwrap_or(0) as f64)
        .collect();

    let wmcp = WeightedMaximumCoverage {
        sets: reindexed_atts,
        weights: reindexed_weights,
        k: 128,
    };

    let mip_start = Instant::now();

    let res = wmcp.solve().unwrap();

    let final_attesters: Vec<Vec<EpochAttesterID>> =
        res.iter().map(|idx| cliqued_atts[*idx].clone()).collect();

    let optimal_reward = calculate_reward(&final_attesters, &instance.reward_function);

    print!("{}", instance.slot.0);

    print!(
        ",{},{}",
        &final_attesters
            .iter()
            .flatten()
            .collect::<HashSet<_>>()
            .len(),
        optimal_reward
    );
    let greedy_reward = calculate_reward(&instance.greedy_solution, &instance.reward_function);
    print!(
        ",{},{},{:.5}",
        &instance
            .greedy_solution
            .iter()
            .flatten()
            .collect::<HashSet<_>>()
            .len(),
        greedy_reward,
        1.0 - (greedy_reward / optimal_reward)
    );

    let end = Instant::now();

    print!(
        ",{},{}",
        end.duration_since(mip_start).as_millis(), // MIP duration
        end.duration_since(start).as_millis()      // all duration
    );

    print!(
        ",{},{},{:.5},{}",
        num_cliques.iter().min().unwrap(),
        num_cliques.iter().max().unwrap(),
        num_cliques.iter().map(|&x| x as f64).sum::<f64>() / (num_cliques.len() as f64),
        wmcp.weights.len(),
    );

    let unique_agg: HashSet<AttestationData> = instance
        .aggregated_attestations
        .iter()
        .map(|att| att.data_root)
        .collect();
    let unique_unagg: HashSet<AttestationData> = instance
        .unaggregated_attestations
        .iter()
        .map(|att| att.data_root)
        .collect();
    let n_unique = unique_agg
        .union(&unique_unagg)
        .collect::<HashSet<&AttestationData>>()
        .len();

    println!(",{}", n_unique) // number of unique attestation data
}

fn group_by_att_data(
    attestations: &Vec<Attestation>,
) -> HashMap<AttestationData, Vec<Attestation>> {
    let mut ret: HashMap<AttestationData, Vec<Attestation>> = HashMap::new();
    for attestation in attestations {
        ret.entry(attestation.data_root)
            .or_default()
            .push(attestation.clone());
    }
    ret
}

fn is_compatible(x: &Attestation, y: &Attestation) -> bool {
    let x_attester_set: HashSet<_> = x.attesting_indices.iter().collect();
    let y_attester_set: HashSet<_> = y.attesting_indices.iter().collect();
    x_attester_set.is_disjoint(&y_attester_set)
}

fn calculate_reward(
    attesters: &[Vec<EpochAttesterID>],
    weights: &HashMap<EpochAttesterID, u64>,
) -> f64 {
    let unique_attesters = attesters.iter().flatten().collect::<HashSet<_>>();

    unique_attesters
        .iter()
        .map(|attester| weights.get(attester).unwrap_or(&0))
        .map(|weight| *weight as f64)
        .sum()
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::{Instance, RawInstance};

    #[test]
    fn scale_test() {
        let instance = Instance::from_file("instances/test.json").unwrap();
        mip_approach(&instance);
    }
}
