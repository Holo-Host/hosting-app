#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    holochain_core_types::{
        entry::Entry, error::HolochainError, hash::HashString, json::JsonString,
        validation::EntryAction,
    },
    holochain_dna::zome::entry_types::Sharing,
};

pub mod entries;
pub mod util;

#[derive(Serialize, Deserialize)]
pub struct Provider {
    name: String,
    fuel_address: String,
}

fn handle_register_app(p: entries::AppConfig) -> JsonString {
    util::make_handler(entries::app_config::register_app)(p)
}

define_zome! {
    entries: [
        entries::app_config::definition(),
        entries::provider::definition()
    ]

    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            register_app: {
                inputs: |config: entries::AppConfig|,
                outputs: |unit: ()|,
                handler: handle_register_app
            }
        }
    }
}
