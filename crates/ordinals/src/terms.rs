use serde::Serializer;

use super::*;

pub fn serialize_option_u128<S>(value: &Option<u128>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  match value {
    Some(v) => serializer.serialize_str(&v.to_string()),
    None => serializer.serialize_none(),
  }
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
pub struct Terms {
  #[serde(serialize_with = "serialize_option_u128")]
  pub amount: Option<u128>,
  #[serde(serialize_with = "serialize_option_u128")]
  pub cap: Option<u128>,
  pub height: (Option<u64>, Option<u64>),
  pub offset: (Option<u64>, Option<u64>),
}
