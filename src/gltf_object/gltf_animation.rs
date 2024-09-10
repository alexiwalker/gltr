use serde_derive::{Deserialize, Serialize};
use crate::prelude::{Extensions, Extras};

use crate::defaults::*;

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct GltfAnimation {
	pub channels: Vec<GltfAnimationChannel>,
	pub sampler: Vec<GltfAnimationSampler>,
	pub name:Option<String>,


	pub extensions:Extensions,
	pub extras:Extras
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfAnimationChannel {

	pub sampler:usize,
	pub target: GltfAnimationChannelTarget,

	pub extensions:Extensions,
	pub extras:Extras

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfAnimationChannelTarget {
	pub node:Option<usize>,
	pub path:String,

	pub extensions:Extensions,
	pub extras:Extras
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfAnimationSampler {

	pub input:usize,

	#[serde(default="animation_sampler_default_interpolation")]
	pub interpolation:String,

	pub output:usize
}