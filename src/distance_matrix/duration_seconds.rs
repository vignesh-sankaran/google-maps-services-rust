use serde::{Deserialize, Deserializer};
use std::time::Duration;

pub fn deserialize<D>(deserializer: D) -> Result<Duration, D::Error>
    where D: Deserializer
{
    Deserialize::deserialize(deserializer).map(Duration::from_secs)
}
