#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use serde_json::Value;

pub mod util;
pub mod service_cycle;

define_zome! {
    entries: [
        service_cycle::definition()
    ]

    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            log_service: {
                inputs: |
                    agent_key: String,
                    request: Value,
                    response: Value,
                    metrics: service_cycle::ServiceMetrics
                |,
                outputs: |unit: ()|,
                handler: service_cycle::log_service
            }
        }
    }
}
