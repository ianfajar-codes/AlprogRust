use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData {
    pub timestamp: BsonDateTime,
    pub value: f64,
}
