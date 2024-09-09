use crate::gltf_object::extras::{Extensions, Extras};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfImage {
	pub uri: Option<String>,

	#[serde(rename = "mimeType")]
	pub mime_type: Option<String>,

	#[serde(rename = "bufferView")]
	pub buffer_view: Option<usize>,

	pub name: Option<String>,
	pub extensions: Extensions,
	pub extras: Extras,

	#[serde(skip)]
	pub(crate) original_index: Option<usize>,
}