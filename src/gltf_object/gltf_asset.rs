use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GltfAsset {
	pub generator: String,
	pub version: String,
}