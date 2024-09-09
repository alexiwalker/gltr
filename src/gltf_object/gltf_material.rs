use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfMaterial {
	name: String,
	//todo i know more material types exist, my test scenes only have this one
	#[serde(rename = "pbrMetallicRoughness")]
	pub pbr_metallic_roughness: Option<GltfMetallicRoughness>,

	#[serde(skip)]
	pub(crate) original_index: Option<usize>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfMetallicRoughness {
	#[serde(rename = "baseColorTexture")]
	pub base_color_texture: Option<GltfBaseColorTexture>,
	pub metallic_factor: Option<usize>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfBaseColorTexture {
	#[serde(rename = "index")]
	pub texture_index: usize,
}


impl GltfMaterial {
	pub fn get_texture_index(&self) -> Option<usize> {
		if self.pbr_metallic_roughness.is_some() {
			let pmr = self.pbr_metallic_roughness.as_ref().unwrap();
			let texture = &pmr.base_color_texture;
			if texture.is_some() {
				let idx = texture.as_ref()?.texture_index;
				return Some(idx);
			}
		}
		None
	}
}