// use boolinator::*;
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::hash::HashString,
    holochain_dna::zome::entry_types::Sharing,
};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)]
pub struct ServiceCycle {
    agent_key: String,
    request_hash: String,
    response_hash: String,
    metrics: ServiceMetrics,
    signature: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceMetrics {
    cpu_time: f32,
    bytes_in: u32,
    bytes_out: u32,
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "service_cycle",
        description: "A log of a single request/response cycle",
        sharing: Sharing::Public,
        native_type: ServiceCycle,

        validation_package: || hdk::ValidationPackageDefinition::Entry,

        validation: |_entry: ServiceCycle, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn log_service(
    agent_key: String,
    request: Value,
    response: Value,
    metrics: ServiceMetrics
) -> ZomeApiResult<HashString> {
    let log = ServiceCycle {
        agent_key,
        metrics,
        request_hash: make_hash(request).to_string(),
        response_hash: make_hash(response).to_string(),
        signature: None,
    };
    let json = serde_json::to_value(log).unwrap();
    hdk::commit_entry("service_cycle", json)
}


use multihash::Hash as Multihash;
use serde::{Serialize, Deserialize};
// TODO: Can't get this to import from another module, so it's here!
fn make_hash<S: Serialize>(value: S) -> HashString {
    HashString::encode_from_serializable(&value, Multihash::SHA2256)
}
