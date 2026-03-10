use crate::enums::CargoKind;
use serde_json::json;

pub struct WSMsg;

impl WSMsg {
    pub fn cargo(cargo_type: CargoKind, id: i32, directory: &str) -> String {
        let value = json!({
            "data": {
                "type": "cargo",
                "cargo_type": cargo_type,
                "id": id,
                "directory": directory,
            }
        });

        serde_json::to_string(&value).unwrap()
    }

    pub fn launch(cargo_amount: usize) -> String {
        let value = json!({
            "data": {
                "type": "launch",
                "cargo_amount": cargo_amount,
            }
        });

        serde_json::to_string(&value).unwrap()
    }

    pub fn weather(is_raining: bool) -> String {
        let value = json!({
            "data": {
                "type": "weather",
                "raining": is_raining,
            }
        });

        serde_json::to_string(&value).unwrap()
    }

    pub fn population(amount: u32) -> String {
        let value = json!({
            "data": {
                "type": "population",
                "amount": amount,
            }
        });

        serde_json::to_string(&value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn cargo_msg() {
        let result = WSMsg::cargo(CargoKind::Drop, 1, "test");
        assert_eq!(
            serde_json::from_str::<Value>(&result).unwrap(),
            serde_json::from_str::<Value>(
                r#"{"type":"cargo","cargo_type":"water","id":1,"directory":"test"}"#
            )
            .unwrap()
        );
    }
}
