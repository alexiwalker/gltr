use crate::buffers::{BufferViewTarget, GltfBufferView, GltfBuffers};
use crate::prelude::*;
use std::fs;
use std::time::Instant;

#[test]
fn test_decode() {
	let content = fs::read_to_string("assets/test_assets/buffers/buffer_1.json").expect("buffer JSON file should exist");

	let buffer_view_b64 = serde_json::from_str::<GltfBuffers>(content.as_str()).expect("should be valid JSON");

	assert!(!buffer_view_b64.0.is_empty());

	let first_buffer = buffer_view_b64.0.first().unwrap().clone();

	let binary_buffer = first_buffer.to_binary().expect("should be valid encoded buffer");

	let l = binary_buffer.byte_length;
	let l2 = binary_buffer.bytes.len();

	assert_eq!(l, l2);
}

#[test]
fn test_views() {
	let content = fs::read_to_string("assets/test_assets/buffers/buffer_1.json")
		.expect("buffer JSON file should exist");

	let views = fs::read_to_string("assets/test_assets/buffers/buffer_views_1.json")
		.expect("buffer view JSON file should exist");

	let buffer_view_b64 = serde_json::from_str::<GltfBuffers>(content.as_str())
		.expect("should be valid JSON");

	let view_list = serde_json::from_str::<Vec<GltfBufferView>>(views.as_str())
		.expect("should be valid JSON");


	let binary_buffers = buffer_view_b64.to_binary()
		.expect("should be valid decoded buffers");

	for x in &view_list {
		let v = binary_buffers.get_view(x);
		assert!(v.is_ok());
	}

	let mut last = view_list.last().unwrap().clone();

	last.byte_length += 1;

	let invalid_range_buffer = binary_buffers.get_view(&last);

	assert!(invalid_range_buffer.is_err());
}
#[test]
fn test_views_2() {
	use rand::Rng;

	let content = fs::read_to_string("assets/test_assets/buffers/buffer_1.json")
		.expect("buffer JSON file should exist");
	let views = fs::read_to_string("assets/test_assets/buffers/buffer_views_1.json")
		.expect("buffer view JSON file should exist");

	let buffer_view_b64 = serde_json::from_str::<GltfBuffers>(content.as_str())
		.expect("should be valid JSON");

	serde_json::from_str::<Vec<GltfBufferView>>(views.as_str())
		.expect("should be valid JSON");
	let binary_buffers = buffer_view_b64.to_binary()
		.expect("should be valid decoded buffers");

	let buff_0_len = binary_buffers.0.first().unwrap().byte_length;
	let mut rng = rand::thread_rng();
	let size = rng.gen_range(0..buff_0_len);

	let offset = buff_0_len - size;

	let mut view = GltfBufferView {
		buffer: 0,
		byte_length: size,
		byte_offset: offset,
		target: Some(BufferViewTarget::ELEMENT_ARRAY_BUFFER),
		original_index: None,
	};

	let v = binary_buffers.get_view(&view);

	assert!(v.is_ok());

	view.byte_offset += 1;

	let v2 = binary_buffers.get_view(&view);

	assert!(v2.is_err())
}


#[test]
pub fn deserialize_full_file() {
	let content = fs::read_to_string("assets/test_assets/cliffs.gltf").expect("scene JSON file should exist");
	let object = GltfObject::try_parse_json_str(content.as_str());
	assert!(object.is_ok());
}


#[test]
pub fn extract_single_object() {
	let content = fs::read_to_string("assets/test_assets/cliffs.gltf").expect("scene JSON file should exist");

	let time_before_parse = Instant::now();
	let object = GltfObject::try_parse_json_str(content.as_str());

	let time_to_parse = time_before_parse.elapsed().as_millis();

	println!("{time_to_parse} to parse original file");

	assert!(object.is_ok());
	let time_before_extract = Instant::now();

	let new = object.unwrap().extract_node(1, GltrExtractFlags::CENTER_OBJECTS | GltrExtractFlags::RECALCULATE_BUFFERS);
	let time_to_extract = time_before_extract.elapsed().as_millis();

	println!("{time_to_extract} to extract node");

	dbg!(&new);
}