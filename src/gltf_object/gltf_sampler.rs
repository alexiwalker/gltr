use serde_derive::{Deserialize, Serialize};
use crate::gltf_object::prelude::*;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfSampler {
	#[serde(rename="magFilter")]
	pub mag_filter:Option<usize>,
	#[serde(rename="minFilter")]
	pub min_filter:Option<usize>,

	#[serde(rename="wrapS")]
	pub wrap_s:Option<usize>,

	#[serde(rename="wrapT")]
	pub wrap_t:Option<usize>,
	pub name:Option<String>,
	pub extras:Extras,
	pub extensions:Extensions,


	#[serde(skip)]
	pub(crate) original_index: Option<usize>,
}