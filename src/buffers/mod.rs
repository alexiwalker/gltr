use base64::prelude::BASE64_STANDARD;
use base64::{DecodeError, Engine};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfBase64Buffer {
	#[serde(rename = "byteLength")]
	pub byte_length: usize,
	pub uri: String,

	#[serde(skip)]
	pub(crate) original_index: Option<usize>,
}


impl GltfBase64Buffer {
	pub fn bytes(&self) -> Result<Vec<u8>, &'static str> {
		let content = &self.uri;

		if !content.starts_with("data:application/octet-stream;base64,") {
			return Err("invalid buffer view encoding: expected RFC2397 encoded application/octet-stream;base64");
		};

		let buf = content.replace("data:application/octet-stream;base64,", "");

		let bytes = BASE64_STANDARD.decode(buf);

		match bytes {
			Ok(b) => {
				Ok(b)
			}
			Err(e) => {
				let err_str = match e {
					DecodeError::InvalidByte(_, _) => {
						"base64 decoding error: DecodeError::InvalidByte"
					}
					DecodeError::InvalidLength(_) => {
						"base64 decoding error: DecodeError::InvalidLength"
					}
					DecodeError::InvalidLastSymbol(_, _) => {
						"base64 decoding error: DecodeError::InvalidLastSymbol"
					}
					DecodeError::InvalidPadding => {
						"base64 decoding error: DecodeError::InvalidPadding"
					}
				};
				Err(err_str)
			}
		}
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GltfBinaryBuffer {
	#[serde(rename = "byteLength")]
	pub byte_length: usize,
	pub bytes: Vec<u8>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GltfBufferView {
	pub buffer: usize,
	#[serde(rename = "byteLength")]
	pub byte_length: usize,
	#[serde(rename = "byteOffset")]
	pub byte_offset: usize,

	//https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.pdf see 5.11.5 bufferView.target
	//enum
	pub target: Option<BufferViewTarget>,

	#[serde(skip)]
	pub(crate) original_index: Option<usize>,

}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BufferViewTarget(usize);
impl BufferViewTarget {
	pub const ARRAY_BUFFER: BufferViewTarget = BufferViewTarget(34962);
	pub const ELEMENT_ARRAY_BUFFER: BufferViewTarget = BufferViewTarget(34963);

	pub fn is_valid(&self) -> bool {
		let v = self.0;
		matches!(v, 34962 | 34963)
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GltfBinaryBuffers(pub Vec<GltfBinaryBuffer>);


#[derive(Deserialize, Serialize, Debug)]
pub struct GltfBuffers(pub Vec<GltfBase64Buffer>);

impl GltfBuffers {
	pub fn new() -> Self {
		GltfBuffers(Vec::new())
	}
}
impl Default for GltfBuffers {
	fn default() -> Self {
		Self::new()
	}
}

impl TryFrom<GltfBuffers> for GltfBinaryBuffers {
	type Error = &'static str;


	fn try_from(value: GltfBuffers) -> Result<Self, Self::Error> {
		let buffers = value.0;
		let mut new_buffers = Vec::with_capacity(buffers.len());

		for x in buffers {
			let new = x.to_binary()?;
			new_buffers.push(new);
		}

		Ok(GltfBinaryBuffers(new_buffers))
	}
}

impl GltfBuffers {
	pub fn empty() -> Self {
		GltfBuffers(vec![])
	}
	pub fn to_binary(self) -> Result<GltfBinaryBuffers, &'static str> {
		GltfBinaryBuffers::try_from(self)
	}
}

impl GltfBinaryBuffers {
	pub fn get_view(&self, view: &GltfBufferView) -> Result<Vec<u8>, &'static str> {
		let target_buffer = view.buffer;

		let buffer = self.0.get(target_buffer);

		match buffer {
			None => {
				Err("No such buffer")
			}
			Some(b) => {
				let length = view.byte_length;
				let offset = view.byte_offset;

				let end_len = offset + length;

				if b.bytes.len() < end_len {
					return Err("not enough bytes");
				};

				let view_bytes = &b.bytes[offset..end_len];

				Ok(view_bytes.to_vec())
			}
		}
	}
}

impl TryFrom<GltfBase64Buffer> for GltfBinaryBuffer {
	type Error = &'static str;

	fn try_from(value: GltfBase64Buffer) -> Result<Self, Self::Error> {
		let bytes = value.bytes()?;

		assert_eq!(bytes.len(), value.byte_length);

		Ok(Self {
			byte_length: value.byte_length,
			bytes,
		})
	}
}

impl GltfBinaryBuffer {
	pub fn from_base64(b64_encoded: GltfBase64Buffer) -> Result<GltfBinaryBuffer, &'static str> {
		GltfBinaryBuffer::try_from(b64_encoded)
	}
}

impl GltfBase64Buffer {
	pub fn to_binary(self) -> Result<GltfBinaryBuffer, &'static str> {
		GltfBinaryBuffer::try_from(self)
	}
}

