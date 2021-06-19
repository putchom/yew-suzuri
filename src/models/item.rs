use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub humanize_name: String,
}