use serde_json::Value;

use crate::support::error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Value,
    pub seconds: u64,
    pub price: f64,
}

impl Task {
    pub fn from_value(value: &Value) -> AppResult<Task> {
        let id = value
            .get("id")
            .cloned()
            .ok_or_else(|| AppError::api("badTask"))?;
        let seconds = number_of(value.get("watch_seconds"))
            .or_else(|| number_of(value.get("duration")))
            .unwrap_or(0.0)
            .max(1.0) as u64;
        let price = number_of(value.get("price")).unwrap_or(0.0);
        Ok(Task { id, seconds, price })
    }
}

pub fn text_of(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(text)) => text.clone(),
        Some(other) => other.to_string(),
        None => String::new(),
    }
}

pub fn number_of(value: Option<&Value>) -> Option<f64> {
    match value? {
        Value::Number(number) => number.as_f64(),
        Value::String(text) => text.trim().parse::<f64>().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_task_keeps_its_id_and_its_watch_time() {
        let value = json!({ "id": 75218740, "watch_seconds": 5, "price": "0.0001000" });
        let task = Task::from_value(&value).unwrap();
        assert_eq!(task.id, json!(75218740));
        assert_eq!(task.seconds, 5);
        assert!((task.price - 0.0001).abs() < 1e-12);
    }

    #[test]
    fn a_missing_watch_time_falls_back_to_the_duration() {
        let value = json!({ "id": 1, "duration": 15 });
        assert_eq!(Task::from_value(&value).unwrap().seconds, 15);
    }

    #[test]
    fn a_task_without_an_id_is_refused() {
        assert!(Task::from_value(&json!({ "watch_seconds": 5 })).is_err());
    }
}
