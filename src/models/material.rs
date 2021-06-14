use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Material {
  pub title: String,
}

impl Default for Material {
  fn default() -> Self {
    Self {
      title: "".to_string(),
    }
  }
}

impl PartialEq for Material {
  fn eq(&self, _other: &Self) -> bool {
    false
  }
}