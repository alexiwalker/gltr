use crate::gltf_object::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfNode {
	pub mesh: Option<usize>,
	pub name: Option<String>,
	pub camera: Option<usize>,
	pub children: Option<Vec<usize>>,
	pub translation: Option<[f32; 3]>,
	pub matrix: Option<[f32; 16]>,
	pub rotation: Option<[f32; 4]>,
	pub scale: Option<[f32; 3]>,
	pub weights: Option<Vec<usize>>,

	pub extras: Extras,
	pub extensions: Extensions,

	#[serde(skip)]
	pub(crate) original_index: Option<usize>,
}