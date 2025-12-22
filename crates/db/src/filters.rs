use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GetAutoPayListFilter {
    #[serde(default)]
    created_by: Option<i64>,
}
