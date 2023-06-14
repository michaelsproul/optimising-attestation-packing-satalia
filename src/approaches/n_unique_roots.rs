use crate::aapp::instance::{AttestationData, Instance};
use std::collections::HashSet;

pub fn n_unique_roots(instance: &Instance) {
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
    println!("{}", n_unique);
}
