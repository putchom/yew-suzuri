use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub humanize_name: String,
}

impl Default for Item {
  fn default() -> Self {
    Self {
      humanize_name: "".to_string(),
    }
  }
}

impl PartialEq for Item {
  fn eq(&self, _other: &Self) -> bool {
      false
  }
}