use serde_derive::{Deserialize, Serialize};
use crate::buffers::{GltfBuffers, GltfBinaryBuffer, GltfBufferView};
use crate::structs::{GltfAsset, GltfMesh, GltfNode, GltfScene};




#[derive(Deserialize, Serialize, Debug)]
pub struct GltfObject {
	pub asset: GltfAsset,
	pub scene: usize,
	#[serde(default = "Vec::new")]
	pub scenes: Vec<GltfScene>,
	#[serde(default = "Vec::new")]
	pub nodes: Vec<GltfNode>,
	#[serde(default = "Vec::new")]
	pub meshes: Vec<GltfMesh>,
	#[serde(default = "Vec::new")]
	pub textures: Vec<GltfTexture>,
	#[serde(default = "Vec::new")]
	pub images:Vec<GltfImage>,
	#[serde(default = "Vec::new")]
	pub accessors:Vec<GltfAccessor>,

	#[serde(rename="bufferViews", default = "Vec::new")]
	pub buffer_views: Vec<GltfBufferView>,

	#[serde(default = "Vec::new")]
	pub samplers: Vec<GltfSampler>,

	#[serde(default = "GltfBuffers::empty")]
	pub buffers: GltfBuffers,
}
 impl GltfObject {
	 pub fn from_str(string:&str) ->Self{
		 serde_json::from_str::<GltfObject>(string).unwrap()
	 }

	 pub fn try_from_str(string:&str) -> serde_json::Result<GltfObject> {
		 serde_json::from_str::<GltfObject>(string)
	 }
 }

pub type Extras = Option<serde_json::Value>;
pub type Extensions = Option<serde_json::Value>;
pub type Object = serde_json::Value;
pub type OptionalObject = Option<serde_json::Value>;


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfTexture {
	sampler:Option<usize>,
	source:Option<usize>,
	name:Option<String>,
	extensions:OptionalObject,
	extras:OptionalObject
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GltfImage {
	 uri:Option<String>,

	#[serde(rename="mimeType")]
	mime_type:Option<String>,

	#[serde(rename="bufferView")]
	buffer_view:Option<usize>,

	name:Option<String>,
	extensions:Extensions,
	extras:Extras,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfAccessor {

	#[serde(rename="bufferView")]
	buffer_view:Option<usize>,


	#[serde(rename="byteOffset")]
	byte_offset:Option<usize>,

	#[serde(rename="componentType")]
	component_type:usize, //todo check valid values? current tests use 5126

	#[serde(default = "bool::default")]
	normalized:bool,

	count:usize,

	r#type:String,

	#[serde(default = "Vec::new")]
	max: Vec<f32>,
	#[serde(default = "Vec::new")]
	min: Vec<f32>,

	sparse: Option<GltfAccessorSparse>,

	name:Option<String>,
	extensions:Extensions,
	extras:Extras
}



#[derive(Deserialize, Serialize, Debug)]
pub struct GltfAccessorSparse {
	count:usize,
	indices:GltfAccessorSparseIndices,
	values:GltfAccessorSparseValues,
	extensions:Extensions,
	extras:Extras
}


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfAccessorSparseIndices {

	#[serde(rename="bufferView")]
	buffer_view:usize,

	#[serde(rename="byteOffset")]
	byte_offset:Option<usize>,

	#[serde(rename="componentType")]
	component_type:usize,

	extras:Extras,
	extensions:Extensions,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GltfAccessorSparseValues {
	#[serde(rename="bufferView")]
	buffer_view:Option<usize>,
	
	#[serde(rename="byteOffset")]
	byte_offset:Option<usize>,

	extras:Extras,

	extensions:Extensions,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfSampler {
	#[serde(rename="magFilter")]
	mag_filter:Option<usize>,
	#[serde(rename="minFilter")]
	min_filter:Option<usize>,

	#[serde(rename="wrapS")]
	wrap_s:Option<usize>,

	#[serde(rename="wrapT")]
	wrap_t:Option<usize>,
	name:Option<String>,
	extras:Extras,
	extensions:Extensions,
}