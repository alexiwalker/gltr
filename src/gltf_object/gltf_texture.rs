use serde_derive::{Deserialize, Serialize};
use crate::gltf_object::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct GltfTexture {
	pub sampler:Option<usize>,
	pub source:Option<usize>,
	pub name:Option<String>,
	pub extensions:Extensions,
	pub extras:Extras
}