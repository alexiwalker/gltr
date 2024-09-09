use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfMesh {
	pub name: String,
	pub primitives: Vec<GltfMeshPrimitive>,

	// #[serde(skip)]
	pub(crate) original_index: Option<usize>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfMeshPrimitive {
	pub attributes: serde_json::Value,

	#[serde(rename = "indices")]
	pub accessor: Option<usize>, // index
	pub material: Option<usize>, // index
	pub mode: Option<usize>, // index
	pub targets: Option<usize>, //todo check valid targets here
	pub extensions: Option<Vec<serde_json::Value>>,
	pub extras: Option<Vec<serde_json::Value>>,

	// #[serde(skip)]
	pub(crate) original_index: Option<usize>,
}
