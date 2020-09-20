mod mesh;

use assimp_sys::*;
pub use mesh::Mesh;
use std::slice;

define_type! {
	struct Scene(&aiScene)
}
impl<'a> Scene<'a> {
	pub fn meshes(&self) -> &[Mesh] {
		unsafe { slice::from_raw_parts(self.mMeshes as _, self.mNumMeshes as _) }
	}
}
impl<'a> Drop for Scene<'a> {
	fn drop(&mut self) {
		unsafe {
			aiReleaseImport(self.0);
		}
	}
}
