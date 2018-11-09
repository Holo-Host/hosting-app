use super::super::util;
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        entry::Entry, error::HolochainError, hash::HashString, json::JsonString,
        validation::EntryAction,
    },
    holochain_dna::zome::entry_types::Sharing,
};
use serde::Serialize;

const ENTRY_TYPE: &str = "app_config";

fn mk_entry<D>(value: D) -> Entry
where
    D: Into<JsonString>,
{
    Entry::new(ENTRY_TYPE.into(), value.into())
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: ENTRY_TYPE,
        description: "Configuration for an app to be hosted",
        sharing: Sharing::Public,
        native_type: AppConfig,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_entry: AppConfig, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct AppConfig {
    dna_hash: HashString,
}

pub fn register_app(config: AppConfig) -> ZomeApiResult<JsonString> {
    let app_config_hash = hdk::commit_entry(&mk_entry(config))?;
    // .map(|hash| json!(hash).into())
    // .unwrap_or_else(|e| e.to_string().into())
    let provider_hash = "silllysilly".into();
    hdk::link_entries(&provider_hash, &app_config_hash, "provided_app")?;
    Ok(app_config_hash.into())
}
