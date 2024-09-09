use std::ops;
use std::ops::Deref;

pub struct GltrExtractFlags(i32);

impl GltrExtractFlags {
	pub const ALL: GltrExtractFlags = GltrExtractFlags(i32::MAX);
	pub const RECALCULATE_BUFFERS: GltrExtractFlags = GltrExtractFlags(1 << 0);
	pub const CENTER_OBJECTS: GltrExtractFlags = GltrExtractFlags(1 << 1);

	pub fn all() -> Self {
		Self::ALL
	}

	pub fn has_flag(&self, flag: GltrExtractFlags) -> bool {
		self.0 & flag.0 != 0
	}
}

impl From<i32> for GltrExtractFlags {
	fn from(value: i32) -> Self {
		Self(value)
	}
}

impl Deref for GltrExtractFlags {
	type Target = i32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::BitOr for GltrExtractFlags {
	type Output = GltrExtractFlags;

	fn bitor(self, rhs: Self) -> Self::Output {
		(self.0 | rhs.0)
			.into()
	}
}

