use serde::{Deserialize, Serialize};

use crate::config::types::subconfigs::kafka::KafkaBroker;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaConfiguration {
    pub brokers: Vec<KafkaBroker>,
}
