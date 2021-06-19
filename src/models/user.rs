use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub name: String,
  pub display_name: Option<String>,
  pub avatar_url: Option<String>,
}