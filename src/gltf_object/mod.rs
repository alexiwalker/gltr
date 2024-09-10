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
mod gltf_camera;
mod gltf_animation;

use crate::buffers::{GltfBufferView, GltfBuffers};

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
	pub use crate::gltf_object::gltf_camera::*;
	pub use crate::gltf_object::gltf_animation::*;
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


#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct GltfSkin {
	#[serde(rename="inverseBindMatrices")]
	pub inverse_bind_matrices:Option<usize>,

	pub skeleton:Option<usize>,

	pub joins:Vec<usize>,

	pub name:Option<String>,

	pub extensions:Extensions,
	pub extras:Extras

}


///see fields in https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.pdf
/// field numbering relative to 5.17 - glTF
#[derive(Deserialize, Serialize, Debug)]
pub struct GltfObject {

	/// 5.17.1 - glTF.extensionsUsed
	/// 
	/// names of glTF extensions used in this asset
	#[serde(rename="extensionsUsed",default = "Vec::new")]
	pub extensions_used: Vec<String>,

	/// 5.17.2 - glTF.extensionsRequired
	/// 
	/// Names of glTF extensions required to properly load this asset.
	#[serde(rename="extensionsRequired",default = "Vec::new")]
	pub extensions_required: Vec<String>,


	/// 5.17.3 - glTF.accessors
	/// 
	/// An array of accessors. An accessor is a typed view into a bufferView
	#[serde(default = "Vec::new")]
	pub accessors: Vec<GltfAccessor>,

	/// 5.17.4 - glTF.animations
	/// 
	/// An array of keyframe animations.
	#[serde(default = "Vec::new")]
	pub animations: Vec<GltfAnimation>,


	/// 5.17.5 - glTF.asset
	/// 
	/// Metadata about the glTF asset.
	pub asset: GltfAsset,

	/// 5.17.6 - glTF.buffers
	/// 
	/// An array of buffers. A buffer points to binary geometry, animation, or skins.
	#[serde(default = "GltfBuffers::empty")]
	pub buffers: GltfBuffers,


	/// 5.17.7 - glTF.bufferViews
	/// 
	/// An array of bufferViews. A bufferView is a view into a buffer generally representing a subset of the buffer.
	#[serde(rename = "bufferViews", default = "Vec::new")]
	pub buffer_views: Vec<GltfBufferView>,

	/// 5.17.8 - glTF.cameras
	/// 
	/// An array of cameras. A camera defines a projection matrix.
	#[serde(default = "Vec::new")]
	pub cameras: Vec<GltfCamera>,

	/// 5.17.9 - glTF.images
	/// 
	/// An array of images. An image defines data used to create a texture.
	#[serde(default = "Vec::new")]
	pub images: Vec<GltfImage>,


	/// 5.17.10 - glTF.materials
	/// 
	/// An array of materials. A material defines the appearance of a primitive.
	#[serde(default = "Vec::new")]
	pub materials: Vec<GltfMaterial>,


	/// 5.17.11 - glTF.meshes
	/// 
	/// An array of meshes. A mesh is a set of primitives to be rendered.
	#[serde(default = "Vec::new")]
	pub meshes: Vec<GltfMesh>,


	/// 5.17.12 - glTF.nodes
	/// 
	/// An array of nodes
	#[serde(default = "Vec::new")]
	pub nodes: Vec<GltfNode>,

	/// 5.17.13 - glTF.samplers
	/// 
	/// An array of samplers. A sampler contains properties for texture filtering and wrapping modes.
	#[serde(default = "Vec::new")]
	pub samplers: Vec<GltfSampler>,

	/// 5.17.14 - glTF.scene
	/// 
	// The index of the default scene. This property MUST NOT be defined, when scenes is undefined.
	pub scene: usize,

	/// 5.17.15 - glTF.scenes
	/// 
	/// An array of scenes
	#[serde(default = "Vec::new")]
	pub scenes: Vec<GltfScene>,

	/// 5.17.16 - glTF.skins
	/// 
	/// An array of skins. A skin is defined by joints and matrices.
	#[serde(default = "Vec::new")]
	pub skins: Vec<GltfSkin>,

	/// 5.17.17 - glTF.textures
	/// 
	/// An array of textures.
	#[serde(default = "Vec::new")]
	pub textures: Vec<GltfTexture>,

	/// 5.17.18 - glTF.extensions
	/// 
	/// JSON object with extension-specific objects.
	pub extensions:Extensions,

	/// 5.17.19 - glTF.extras
	/// 
	/// Application-specific data.
	pub extras:Extras

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
			extensions_used: vec![],
			asset: GltfAsset {
				generator: "Gltr Library v0.0.1".to_string(),
				version: "2.0".to_string(),
			},
			scene: 0,
			scenes: vec![],
			nodes: vec![],
			meshes: vec![],
			textures: vec![],
			extensions: None,
			images: vec![],
			accessors: vec![],
			materials: vec![],
			buffer_views: vec![],
			samplers: vec![],
			buffers: GltfBuffers::default(),
			extensions_required: vec![],
			animations: vec![],
			cameras: vec![],
			skins: vec![],
			extras: None,
		}
	}
}

