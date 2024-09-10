#[cfg(test)]
mod tests;
pub mod buffers;
mod gltf_object;
mod ops;

pub mod prelude {
	pub use crate::buffers::*;
	pub use crate::gltf_object::prelude::*;
	pub use crate::gltf_object::*;
}


#[allow(dead_code)]
pub(crate) mod defaults {
	pub(crate) const fn default_1() -> usize {
		1
	}
	pub(crate) const fn default_0() -> usize {
		0
	}

	pub(crate) const fn default_1f() -> f32 {
		1f32
	}
	pub(crate) const fn default_0f() -> f32 {
		0f32
	}

	pub(crate) const fn default_f32_vec4_1() -> [f32; 4] {
		[1f32, 1f32, 1f32, 1f32]
	}

	pub(crate) const fn default_f32_vec_1() -> [f32; 3] {
		[1f32, 1f32, 1f32]
	}

	pub(crate) const fn default_f32_vec4_0() -> [f32; 4] {
		[0f32, 0f32, 0f32, 0f32]
	}

	pub(crate) const fn default_f32_vec_0() -> [f32; 3] {
		[0f32, 0f32, 0f32]
	}


	pub(crate) fn material_default_alpha_mode() -> String {
		"OPAQUE".to_string()
	}

	pub(crate) fn animation_sampler_default_interpolation() -> String {
		"LINEAR".to_string()
	}
}