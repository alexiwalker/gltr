use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]

pub struct GltfScene {
	pub name: String,
	pub nodes: Vec<usize>,
}