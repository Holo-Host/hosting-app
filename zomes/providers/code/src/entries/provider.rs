use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{hash::HashString, validation::EntryAction},
    holochain_dna::zome::entry_types::Sharing,
};

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "provider",
        description: "An app provider",
        sharing: Sharing::Public,
        native_type: Provider,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_entry: Provider, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

#[derive(Serialize, Deserialize)]
pub struct Provider {
    /// display name
    name: String,

    /// holofuel address
    transactor_id: HashString,
}
