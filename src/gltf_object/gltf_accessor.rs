use serde_derive::{Deserialize, Serialize};
use crate::gltf_object::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfAccessor {

	#[serde(rename="bufferView")]
	pub buffer_view:Option<usize>,


	#[serde(rename="byteOffset")]
	pub byte_offset:Option<usize>,

	#[serde(rename="componentType")]
	pub component_type:usize, //todo check valid values? current tests use 5126

	#[serde(default = "bool::default")]
	pub normalized:bool,

	pub count:usize,

	pub r#type:String,

	#[serde(default = "Vec::new")]
	pub max: Vec<f32>,
	#[serde(default = "Vec::new")]
	pub min: Vec<f32>,

	pub sparse: Option<GltfAccessorSparse>,

	pub name:Option<String>,
	pub extensions:Extensions,
	pub extras:Extras
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfAccessorSparse {
	pub count:usize,
	pub indices:GltfAccessorSparseIndices,
	pub values:GltfAccessorSparseValues,
	pub extensions:Extensions,
	pub extras:Extras
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfAccessorSparseIndices {

	#[serde(rename="bufferView")]
	pub buffer_view:usize,

	#[serde(rename="byteOffset")]
	pub byte_offset:Option<usize>,

	#[serde(rename="componentType")]
	pub component_type:usize,

	pub extras:Extras,
	pub extensions:Extensions,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfAccessorSparseValues {
	#[serde(rename="bufferView")]
	buffer_view:Option<usize>,

	#[serde(rename="byteOffset")]
	byte_offset:Option<usize>,

	extras:Extras,

	extensions:Extensions,
}
