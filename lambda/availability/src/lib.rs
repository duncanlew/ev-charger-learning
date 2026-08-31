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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_mock_availability() {
        let result = check_availability();
        assert!(result.available);
        assert_eq!(result.message, "Charger is mock available");
    }
}
