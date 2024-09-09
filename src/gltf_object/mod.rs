mod extras;
mod gltf_image;
mod gltf_accessor;
mod gltf_texture;
mod gltf_sampler;
mod gltf_asset;
mod gltf_scene;
mod gltf_node;
mod gltf_material;
mod gltf_mesh;
pub mod extract_flags;

use std::collections::HashMap;
use std::ops::Deref;
use serde_derive::{Deserialize, Serialize};
use extract_flags::GltrExtractFlags;
use gltf_accessor::GltfAccessor;
use gltf_image::GltfImage;
use gltf_sampler::GltfSampler;
use gltf_texture::GltfTexture;
use crate::buffers::{GltfBufferView, GltfBuffers};

pub mod prelude {
	pub use crate::gltf_object::extras::*;
	pub use crate::gltf_object::gltf_scene::*;
	pub use crate::gltf_object::gltf_asset::*;
	pub use crate::gltf_object::gltf_accessor::*;
	pub use crate::gltf_object::gltf_image::*;
	pub use crate::gltf_object::gltf_material::*;
	pub use crate::gltf_object::gltf_mesh::*;
	pub use crate::gltf_object::gltf_node::*;
	pub use crate::gltf_object::gltf_sampler::*;
	pub use crate::gltf_object::gltf_scene::*;
	pub use crate::gltf_object::gltf_texture::*;
	pub use crate::buffers as gltf_buffers;
	pub use crate::gltf_object::extract_flags as gltf_extract_flags;
}


use crate::gltf_object::prelude::*;


#[derive(Deserialize, Serialize, Debug)]
pub enum GltrError {
	InvalidJson(usize, usize, String), //line,column,message
	ConstraintViolation(String),
	InvalidIndex(&'static str, usize),
}

pub type GltrResult<T> = Result<T, GltrError>;

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
	pub images: Vec<GltfImage>,
	#[serde(default = "Vec::new")]
	pub accessors: Vec<GltfAccessor>,

	#[serde(rename = "bufferViews", default = "Vec::new")]
	pub buffer_views: Vec<GltfBufferView>,

	#[serde(default = "Vec::new")]
	pub samplers: Vec<GltfSampler>,

	#[serde(default = "GltfBuffers::empty")]
	pub buffers: GltfBuffers,
}
impl GltfObject {
	pub fn parse_json_str(string: &str) -> Self {
		serde_json::from_str::<GltfObject>(string).unwrap()
	}

	pub fn try_parse_json_str(string: &str) -> serde_json::Result<GltfObject> {
		serde_json::from_str::<GltfObject>(string)
	}

	pub fn extract_node(&self, idx: usize, flags: GltrExtractFlags) -> GltrResult<Self> {
		let node = self.nodes.get(idx);

		let node = match node {
			None => {
				return Err(GltrError::InvalidIndex("Node", idx))
			}
			Some(node) => {
				node.clone()
			}
		};

		let mut new_object = GltfObject::new();

		let mut node = node.clone();


		if flags.has_flag(GltrExtractFlags::CENTER_OBJECTS) && node.translation.is_some() {
			node.translation = Some([0f32, 0f32, 0f32])
		}


		let mesh_idx = &node.mesh;

		if mesh_idx.is_some() {
			let mesh_idx = mesh_idx.unwrap();

			let mesh = self.meshes.get(mesh_idx);

			let mesh = match mesh {
				None => {
					return Err(GltrError::InvalidIndex("Mesh", mesh_idx))
				}
				Some(mesh) => {
					mesh.clone()
					// cloning a mesh here is 'cheap' (ish) as it is only metadata
					// this field does NOT include the actual vertices, materials etc
				}
			};

			let primitives = mesh.primitives.clone();

			//maps are <originalIndex,newIndex>
			let mut accessor_indices_remap = HashMap::<usize, usize>::new();
			let mut material_remap = HashMap::<usize, usize>::new();
			let mut buffer_view_remap = HashMap::<usize, usize>::new();

			let mut current_accessor_index = 0;
			let mut current_material_index = 0;
			let mut current_buffer_index = 0;

			let mut new_primitives: Vec<GltfMeshPrimitive> = Vec::new();
			for x in primitives {
				let accessor = x.accessor;
				let material = x.material;

				let mut new_primitive = x.clone();

				if accessor.is_some() {
					let accessor_index = accessor.unwrap();

					let mut accessor = match self.accessors.get(accessor_index) {
						None => {
							return Err(GltrError::InvalidIndex("Indices", mesh_idx))
						}
						Some(a) => {
							a.clone()
						}
					};


					if accessor.buffer_view.is_some() {
						let original_buffer_index = (&accessor.buffer_view).unwrap();

						if let std::collections::hash_map::Entry::Vacant(e) = buffer_view_remap.entry(original_buffer_index) {
							e.insert(current_buffer_index);
							current_buffer_index+=1;
						}


						let new_view = *buffer_view_remap.get(&original_buffer_index).unwrap();
						accessor.buffer_view = Some(new_view);

					}
					
					
					

				}
				
				
				if material.is_some() {
					let material_index = material.unwrap();

					let mut material = match self.accessors.get(material_index) {
						None => {
							return Err(GltrError::InvalidIndex("material", mesh_idx))
						}
						Some(a) => {
							a.clone()
						}
					};
					
					

				}

				new_primitives.push(new_primitive);
			}
		}

		new_object.scene = self.scene;
		new_object.nodes.push(node);

		Ok(new_object)
	}

	pub fn new() -> Self {
		GltfObject {
			asset: GltfAsset {
				generator: "Gltr Library v0.0.1".to_string(),
				version: "2.0".to_string(),
			},
			scene: 0,
			scenes: vec![],
			nodes: vec![],
			meshes: vec![],
			textures: vec![],
			images: vec![],
			accessors: vec![],
			buffer_views: vec![],
			samplers: vec![],
			buffers: GltfBuffers::default(),
		}
	}
}

