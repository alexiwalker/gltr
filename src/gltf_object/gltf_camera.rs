use serde_derive::{Deserialize, Serialize};
use crate::prelude::{Extensions, Extras};

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct GltfCamera {
	pub orthographic:Option<GltfCameraOrthographic>,
	pub perspective:Option<GltfCameraPerspective>,

	#[serde(rename="type")]
	pub camera_type: String,

	pub name:Option<String>,
	pub extensions:Extensions,
	pub extras:Extras
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfCameraOrthographic {

	#[serde(rename="xmag")]
	pub x_magnitude:usize,

	#[serde(rename="ymag")]
	pub y_magnitude:usize,

	#[serde(rename="zfar")]
	pub z_far:usize,

	#[serde(rename="znear")]
	pub z_near:usize,

	pub extensions:Extensions,
	pub extras:Extras
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfCameraPerspective {
	#[serde(rename="aspectRatio")]
	pub aspect_ratio:Option<usize>,

	#[serde(rename="yfov")]
	pub y_fov: usize,
	#[serde(rename="zfar")]
	pub z_far:Option<usize>,
	#[serde(rename="znear")]
	pub z_near:usize,

	pub extensions:Extensions,
	pub extras:Extras
}