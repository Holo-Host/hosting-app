use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{hash::HashString, validation::EntryAction},
    holochain_dna::zome::entry_types::Sharing,
};
use serde_json::{self, Value};

use super::util;

const ENTRY_NAME: &str = "service_cycle";

#[derive(Serialize, Deserialize)]
pub struct ServiceCycle {
    agent_key: String,
    request_hash: HashString,
    response_hash: HashString,
    metrics: ServiceMetrics,
    signature: Option<String>,
}

/// The data which the client will sign
#[derive(Serialize, Deserialize)]
pub struct SignedData {
    // metrics: ServiceMetrics,
    response_hash: HashString,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceMetrics {
    cpu_time: f32,
    bytes_in: u32,
    bytes_out: u32,
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: ENTRY_NAME,
        description: "A log of a single request_payload/response_payload cycle",
        sharing: Sharing::Public,
        native_type: ServiceCycle,

        validation_package: || hdk::ValidationPackageDefinition::Entry,

        validation: |post: ServiceCycle, ctx: hdk::ValidationData| {
            // NB: put it in a separate function because errors in this closure
            // are so dang hard to debug
            validation(post, ctx)
        }
    )
}

fn validation(log: ServiceCycle, ctx: hdk::ValidationData) -> Result<(), String> {
    match (log.signature.clone(), ctx.action) {
        (Some(signature), EntryAction::Modify) => {
            let data = SignedData {
                response_hash: log.response_hash.clone(),
            };
            serde_json::to_string(&data)
                .map_err(|e| e.to_string())
                .and_then(|data_string| {
                    hdk::verify_signature(signature, data_string, log.agent_key)
                        .map(|_| ())
                        .map_err(|e| e.to_string())
                })
        }
        (None, EntryAction::Commit) => Ok(()),
        (_, EntryAction::Delete) => Err("Can't delete service logs!".to_string()),
        _ => Err("Invalid service log action".to_string()),
    }
}

pub fn log_service(
    agent_key: String,
    request_payload: Value,
    response_payload: Value,
    metrics: ServiceMetrics,
) -> ZomeApiResult<HashString> {
    let log = ServiceCycle {
        agent_key,
        metrics,
        request_hash: util::make_hash(request_payload),
        response_hash: util::make_hash(response_payload),
        signature: None,
    };
    let json = serde_json::to_value(log).unwrap();
    hdk::commit_entry(ENTRY_NAME, json)
}

pub fn add_signature(entry_hash: HashString, signature: String) -> ZomeApiResult<HashString> {
    let mut entry: ServiceCycle =
        hdk::get_entry(entry_hash.clone())?.ok_or(ZomeApiError::HashNotFound)?;
    entry.signature = Some(signature);
    let updated = serde_json::to_value(entry).map_err(|e| ZomeApiError::Internal(e.to_string()))?;
    hdk::update_entry(ENTRY_NAME, updated, entry_hash)
}
