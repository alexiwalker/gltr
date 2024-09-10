use serde_derive::{Deserialize, Serialize};
use crate::prelude::{Extensions, Extras};

use crate::defaults::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GltfMaterial {
	name: String,
	pub pbr_metallic_roughness: Option<GltfMetallicRoughness>,

	pub normal_texture:Option<GltfNormalTexture>,

	pub occlusion_texture:Option<GltfOcclusionTexture>,

	pub emissive_texture:Option<GltfEmissiveTexture>,

	pub emissive_factor: Option<[usize; 3]>,

	#[serde(default="material_default_alpha_mode")]
	pub alpha_mode:String,

	pub alpha_cutoff:f32,

	pub extensions:Extensions,
	pub extras:Extras,

	#[serde(skip)]
	pub(crate) original_index: Option<usize>,
}



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfNormalTexture {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfOcclusionTexture {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfEmissiveTexture {

}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfMetallicRoughness {
	
	#[serde(default="default_f32_vec4_1")]
	pub base_color_factor:[f32;4],
	
	#[serde(rename = "baseColorTexture")]
	pub base_color_texture: Option<GltfTextureInfo>,
	pub metallic_factor: Option<f32>,
	pub roughness_factor:Option<f32>,
	pub metallic_roughness_texture:Option<GltfTextureInfo>,
	pub extensions:Extensions,
	pub extras:Extras,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfTextureInfo {
	#[serde(rename = "index")]
	pub texture_index: usize,
	
	#[serde(default="default_0")]
	pub tex_coord:usize,

	pub extensions:Extensions,
	pub extras:Extras,
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