use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub humanize_name: String,
}

impl Clone for Item {
  fn clone(&self) -> Self {
    Self {
      humanize_name: self.humanize_name.clone(),
    }
  }
}

impl Default for Item {
  fn default() -> Self {
    Self {
      humanize_name: "".into(),
    }
  }
}

impl PartialEq for Item {
  fn eq(&self, other: &Self) -> bool {
    return self.humanize_name == other.humanize_name;
  }
}