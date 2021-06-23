use serde::{
  Deserialize,
  Serialize
};
use crate::models::user::User;

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Material {
  pub title: String,
  pub description: Option<String>,
  pub user: User,
}