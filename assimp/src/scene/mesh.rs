use assimp_sys::*;
use std::{iter, slice};

define_type! {
	struct Mesh(&aiMesh)
}
impl<'a> Mesh<'a> {
	pub fn vertices(&self) -> &[[f32; 3]] {
		unsafe { slice::from_raw_parts(self.mVertices as _, self.mNumVertices as _) }
	}

	pub fn normals(&self) -> &[[f32; 3]] {
		unsafe { slice::from_raw_parts(self.mNormals as _, self.mNumVertices as _) }
	}

	pub fn texture_coords(&self) -> impl Iterator<Item = &[[f32; 3]]> {
		let mut channel = 0;
		iter::from_fn(move || {
			if channel >= AI_MAX_NUMBER_OF_TEXTURECOORDS {
				return None;
			}

			let coords = self.mTextureCoords[channel];
			if coords.is_null() {
				None
			} else {
				channel += 1;
				Some(unsafe { slice::from_raw_parts(coords as _, self.mNumVertices as _) })
			}
		})
	}

	pub fn faces(&self) -> &[Face] {
		unsafe { slice::from_raw_parts(self.mFaces as _, self.mNumFaces as _) }
	}
}

define_type! {
	struct Face(&aiFace)
}
impl<'a> Face<'a> {
	pub fn indices(&self) -> &[u32] {
		unsafe { slice::from_raw_parts(self.mIndices as _, self.mNumIndices as _) }
	}
}
