use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Material {
  pub title: String,
}

impl Clone for Material {
  fn clone(&self) -> Self {
    Self {
      title: self.title.clone(),
    }
  }
}

impl Default for Material {
  fn default() -> Self {
    Self {
      title: "".into(),
    }
  }
}

impl PartialEq for Material {
  fn eq(&self, other: &Self) -> bool {
    return self.title == other.title;
  }
}