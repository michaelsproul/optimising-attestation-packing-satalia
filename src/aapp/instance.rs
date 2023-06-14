use primitive_types::H256;
use serde::{de, Deserialize, Deserializer};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

/// Utility method for deserialization of String into a generic T: FromStr
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub type EpochID = u64;

/// A type representing a slot ID
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct SlotID(#[serde(deserialize_with = "from_str")] pub(crate) u64);

pub type AttesterID = u64;

pub type EpochAttesterID = (EpochID, AttesterID);

pub type AttestationData = H256;

#[derive(Debug, Deserialize, Clone)]
pub struct RawAttestation {
    pub attesting_indices: Vec<AttesterID>,
    pub data_root: AttestationData,
    pub slot: Option<SlotID>,
    #[serde(deserialize_with = "from_str")]
    pub index: u64,
}

#[derive(Clone)]
pub struct Attestation {
    pub attesting_indices: Vec<EpochAttesterID>,
    pub data_root: AttestationData,
}

#[derive(Debug, Deserialize)]
pub struct RawInstance {
    slot: SlotID,
    // parent_slot: SlotID,
    unaggregated_attestations: HashMap<SlotID, Vec<RawAttestation>>,
    aggregated_attestations: HashMap<SlotID, Vec<RawAttestation>>,
    reward_function: HashMap<EpochID, HashMap<AttesterID, u64>>,
    greedy_solution: Vec<RawAttestation>,
}

pub struct Instance {
    pub slot: SlotID,
    pub aggregated_attestations: Vec<Attestation>,
    pub unaggregated_attestations: Vec<Attestation>,
    pub reward_function: HashMap<EpochAttesterID, u64>,
    pub attestation_data: Vec<AttestationData>,
    pub greedy_solution: Vec<Vec<EpochAttesterID>>,
}

impl Attestation {
    fn from_raw(original: &RawAttestation, epoch: EpochID) -> Self {
        Self {
            attesting_indices: original
                .attesting_indices
                .iter()
                .map(|attester| (epoch, *attester))
                .collect(),
            data_root: original.data_root,
        }
    }
}

impl From<RawInstance> for Instance {
    fn from(original: RawInstance) -> Self {
        let mut attestation_data = HashSet::new();
        original
            .aggregated_attestations
            .values()
            .flatten()
            .map(|x| x.data_root)
            .collect_into(&mut attestation_data);

        Instance {
            slot: original.slot,
            aggregated_attestations: original
                .aggregated_attestations
                .iter()
                .flat_map(|(slot, raw_atts)| {
                    let epochs = std::iter::repeat(slot.0 / 32);
                    raw_atts
                        .iter()
                        .zip(epochs)
                        .map(|(raw_att, epoch)| Attestation::from_raw(raw_att, epoch))
                })
                .collect(),
            unaggregated_attestations: original
                .unaggregated_attestations
                .iter()
                .flat_map(|(slot, raw_atts)| {
                    let epochs = std::iter::repeat(slot.0 / 32);
                    raw_atts
                        .iter()
                        .zip(epochs)
                        .map(|(raw_att, epoch)| Attestation::from_raw(raw_att, epoch))
                })
                .collect(),
            reward_function: original
                .reward_function
                .iter()
                .flat_map(|(epoch, epoch_rewards)| {
                    epoch_rewards
                        .iter()
                        .map(|(attester, reward)| ((*epoch, *attester), *reward))
                })
                .collect(),
            attestation_data: attestation_data
                .into_iter()
                .collect::<Vec<AttestationData>>(),
            greedy_solution: original
                .greedy_solution
                .iter()
                .map(|attestation| {
                    let epoch = attestation.slot.clone().unwrap().0 / 32;
                    attestation
                        .attesting_indices
                        .iter()
                        .map(|attester| (epoch, *attester))
                        .collect()
                })
                .collect(),
        }
    }
}

impl Instance {
    pub fn from_file(filename: &str) -> Result<Instance, std::io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let instance: RawInstance = serde_json::from_reader(reader)?;
        Ok(instance.into())
    }
}
