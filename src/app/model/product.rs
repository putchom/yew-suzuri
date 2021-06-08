use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
  pub title: String
}