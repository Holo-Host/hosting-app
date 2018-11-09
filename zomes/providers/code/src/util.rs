use hdk::{
    self,
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        entry::Entry, error::HolochainError, hash::HashString, json::JsonString,
        validation::EntryAction,
    },
};
use multihash::Hash as Multihash;
use serde::{Deserialize, Serialize};
use serde_json::Value;

type FuncDef<P, V, E> = fn(P) -> Result<V, E>;

/// Take a function that returns a Result<> of two Into<JsonString> things
/// and produces a closure whose output is just a JsonString
pub fn make_handler<P, V, E>(func: FuncDef<P, V, E>) -> (impl Fn(P) -> JsonString)
where
    V: Into<JsonString>,
    E: Into<JsonString>,
{
    move |params: P| {
        let result = func(params);
        let output: JsonString = match result {
            Ok(val) => val.into(),
            Err(err) => err.into(),
        };
        output
    }
}
