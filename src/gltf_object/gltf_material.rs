use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GltfMaterial {
	name: String,
	//todo i know more material types exist, my test scenes only have this one
	#[serde(rename = "pbrMetallicRoughness")]
	pub pbr_metallic_roughness: Option<GltfMetallicRoughness>,

	#[serde(skip)]

	pub(crate) original_index: Option<usize>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfMetallicRoughness {
	#[serde(rename = "baseColorTexture")]
	pub base_color_texture: Option<GltfBaseColorTexture>,
	pub metallic_factor: usize,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfBaseColorTexture {
	pub index: usize,
}
