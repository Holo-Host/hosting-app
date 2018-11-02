#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use hdk::{
    holochain_dna::zome::entry_types::Sharing,
};

#[derive(Serialize, Deserialize)]
pub struct Provider {
    name: String,
    fuel_address: String,
}

pub fn handle_add_app() {

}

define_zome! {
    entries: [
        entry!(
            name: "provider",
            description: "An app provider",
            sharing: Sharing::Public,
            native_type: Provider,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |entry: Provider, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            add_app: {
                inputs: | |,
                outputs: |unit: ()|,
                handler: handle_add_app
            }
        }
    }
}
