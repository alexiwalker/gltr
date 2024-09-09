use serde_derive::{Deserialize, Serialize};

pub type Extras = Option<serde_json::Value>;
pub type Extensions = Option<serde_json::Value>;
pub type OptionalObject = Option<serde_json::Value>;

