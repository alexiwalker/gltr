use crate::gltf_object::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfTexture {
	#[serde(rename = "sampler")]
	pub sample_index: Option<usize>,

	#[serde(rename = "source")]
	pub source_image_index: Option<usize>,
	pub name: Option<String>,
	pub extensions: Extensions,
	pub extras: Extras,

	#[serde(skip)]
	pub(crate) original_index: Option<usize>,
}