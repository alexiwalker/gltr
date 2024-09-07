use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfAsset {
	pub generator: String,
	pub version: String,
}
#[derive(Deserialize, Serialize, Debug)]

pub struct GltfScene {
	pub name: String,
	pub nodes: Vec<usize>,
}
#[derive(Deserialize, Serialize, Debug)]

pub struct GltfNode {
	pub mesh: usize,
	pub name: String,
	pub translation: [f32; 3],
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GltfMaterials {
	name: String,
	//todo i know more material types exist, my test scenes only have this one
	#[serde(rename = "pbrMetallicRoughness")]
	pub pbr_metallic_roughness: Option<GltfMetallicRoughness>,
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


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfMesh {
	pub name: String,
	pub primitives: Vec<GltfMeshPrimitive>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfMeshPrimitive {
	pub attributes: serde_json::Value,
	pub indices: Option<usize>, // index
	pub material: Option<usize>, // index
	pub mode: Option<usize>, // index
	pub targets: Option<usize>, //todo check valid targets here
	pub extensions:Option<Vec<serde_json::Value>>,
	pub extras:Option<Vec<serde_json::Value>>,
}
