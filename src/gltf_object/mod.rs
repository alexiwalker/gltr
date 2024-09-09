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
mod extract_flags;

use crate::buffers::{GltfBufferView, GltfBuffers};
use extract_flags::GltrExtractFlags;
use gltf_accessor::GltfAccessor;
use gltf_image::GltfImage;
use gltf_sampler::GltfSampler;
use gltf_texture::GltfTexture;
use serde_derive::{Deserialize, Serialize};


#[allow(unused_imports)]
pub mod prelude {
	pub mod ops {
		pub use crate::ops::*;
	}

	pub use crate::buffers as gltf_buffers;
	pub use crate::gltf_object::extract_flags::*;
	pub use crate::gltf_object::extras::*;
	pub use crate::gltf_object::gltf_accessor::*;
	pub use crate::gltf_object::gltf_asset::*;
	pub use crate::gltf_object::gltf_image::*;
	pub use crate::gltf_object::gltf_material::*;
	pub use crate::gltf_object::gltf_mesh::*;
	pub use crate::gltf_object::gltf_node::*;
	pub use crate::gltf_object::gltf_sampler::*;
	pub use crate::gltf_object::gltf_scene::*;
	pub use crate::gltf_object::gltf_scene::*;
	pub use crate::gltf_object::gltf_texture::*;
}


use crate::gltf_object::prelude::*;
use crate::ops::VecExt;

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

	#[serde(default = "Vec::new")]
	pub materials: Vec<GltfMaterial>,

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

		node.original_index = Some(idx);


		if flags.has_flag(GltrExtractFlags::CENTER_OBJECTS) && node.translation.is_some() {
			node.translation = Some([0f32, 0f32, 0f32])
		}


		let mesh_idx = &node.mesh;

		if mesh_idx.is_some() {
			let mut mesh = match self.meshes.get(mesh_idx.unwrap()) {
				None => {
					return Err(GltrError::InvalidIndex("Mesh", mesh_idx.unwrap()))
				}
				Some(mesh) => {
					mesh.clone()
				}
			};

			mesh.original_index = *mesh_idx;


			for x in &mesh.primitives {
				let accessor_index = x.accessor;
				let material_index = x.material;

				if accessor_index.is_some() {
					let mut accessor = match self.accessors.get(accessor_index.unwrap()) {
						None => {
							return Err(GltrError::InvalidIndex("Accessor", mesh_idx.unwrap()))
						}
						Some(accessor) => {
							accessor.clone()
						}
					};

					accessor.original_index = accessor_index;

					new_object.accessors.push_if_no_match(accessor.clone(), |x| x.original_index == mesh.original_index);
				}


				if material_index.is_some() {
					let mut material = match self.materials.get(material_index.unwrap()) {
						None => {
							return Err(GltrError::InvalidIndex("Material", mesh_idx.unwrap()))
						}
						Some(material) => {
							material.clone()
						}
					};

					material.original_index = material_index;

					new_object.materials.push_if_no_match(material.clone(), |x| {
						x.original_index == material.original_index
					});
				}
			}

			new_object.meshes.push_if_no_match(mesh.clone(), |x| {
				x.original_index == mesh.original_index
			})
		}


		for x in &new_object.materials {
			let texture_index = x.get_texture_index();

			if texture_index.is_some() {
				let mut texture = match self.textures.get(texture_index.unwrap()) {
					None => {
						return Err(GltrError::InvalidIndex("Texture", texture_index.unwrap()))
					}
					Some(tex) => {
						tex.clone()
					}
				};
				texture.original_index = texture_index;
				new_object.textures.push_if_no_match(texture.clone(), |x| x.original_index == texture.original_index)
			}
		}


		for x in &new_object.textures {
			let image_idx = x.source_image_index;
			if image_idx.is_some() {
				let mut image = match self.images.get(image_idx.unwrap()) {
					None => {
						return Err(GltrError::InvalidIndex("Image", x.source_image_index.unwrap()))
					}
					Some(image) => {
						image.clone()
					}
				};

				image.original_index = image_idx;
				new_object.images.push_if_no_match(image.clone(), |x| x.original_index == image.original_index)
			}

			let sampler_index = x.sample_index;
			if sampler_index.is_some() {
				let mut sampler = match self.samplers.get(sampler_index.unwrap()) {
					None => {
						return Err(GltrError::InvalidIndex("Sampler", x.sample_index.unwrap()))
					}
					Some(sampler) => {
						sampler.clone()
					}
				};

				sampler.original_index = sampler_index;
				new_object.samplers.push_if_no_match(sampler.clone(), |x| x.original_index == sampler.original_index)
			}
		}

		for x in &new_object.accessors {
			let buffer_view_index = x.buffer_view;
			if buffer_view_index.is_some() {
				let mut buffer_view = match self.buffer_views.get(buffer_view_index.unwrap()) {
					None => {
						return Err(GltrError::InvalidIndex("buffer_view", mesh_idx.unwrap()))
					}
					Some(bv) => {
						bv.clone()
					}
				};

				buffer_view.original_index = buffer_view_index;
				new_object.buffer_views.push_if_no_match(buffer_view.clone(), |x| x.original_index == buffer_view.original_index)
			}
		}


		for x in &new_object.images {
			let buffer_view_index = x.buffer_view;
			if buffer_view_index.is_some() {
				let mut buffer_view = match self.buffer_views.get(buffer_view_index.unwrap()) {
					None => {
						return Err(GltrError::InvalidIndex("buffer_view", mesh_idx.unwrap()))
					}
					Some(bv) => {
						bv.clone()
					}
				};

				buffer_view.original_index = buffer_view_index;
				new_object.buffer_views.push_if_no_match(buffer_view.clone(), |x| x.original_index == buffer_view.original_index)
			}
		}


		for x in &new_object.buffer_views {
			let buffer_index = x.buffer;

			let mut has = false;
			for x in &new_object.buffers.0 {
				if x.original_index.is_some() && x.original_index.unwrap() == buffer_index {
					has = true;
				}
			}

			if !has {
				let buffer = match self.buffers.0.get(buffer_index) {
					None => {
						return Err(GltrError::InvalidIndex("buffer", mesh_idx.unwrap()))
					}
					Some(b) => {
						b.clone()
					}
				};

				new_object.buffers.0.push(buffer)
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
			materials: vec![],
			buffer_views: vec![],
			samplers: vec![],
			buffers: GltfBuffers::default(),
		}
	}
}

