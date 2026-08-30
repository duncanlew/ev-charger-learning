use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    pub available: bool,
    pub message: String,
}

pub fn check_availability() -> AvailabilityResponse {
    AvailabilityResponse {
        available: true,
        message: String::from("Charger is mock available"),
    }
}
