use crate::scene::Scene;
use assimp_sys::*;
use std::{
	ffi::{CStr, CString},
	ptr::null_mut,
	str,
};

pub struct Importer {
	property_store: *mut aiPropertyStore,
	flags: aiPostProcessSteps,
}
impl Importer {
	pub fn new() -> Self {
		let property_store = unsafe { aiCreatePropertyStore() };
		let flags = 0;
		Self { property_store, flags }
	}

	pub fn read_file(&self, file: &str) -> Result<Scene, &str> {
		let file = CString::new(file).unwrap();
		let scene =
			unsafe { aiImportFileExWithProperties(file.as_ptr(), self.flags as _, null_mut(), self.property_store) };
		if scene.is_null() {
			let err = unsafe { aiGetErrorString() };
			if err.is_null() {
				Err("Unknown error")
			} else {
				let err = unsafe { CStr::from_ptr(err) };
				Err(str::from_utf8(err.to_bytes()).unwrap())
			}
		} else {
			Ok(Scene::from_raw(scene))
		}
	}

	pub fn triangulate(&mut self, enable: bool) {
		self.set_flag(aiPostProcessSteps_aiProcess_Triangulate, enable);
	}

	fn set_flag(&mut self, flag: i32, enable: bool) {
		if enable {
			self.flags |= flag;
		} else {
			self.flags &= !flag;
		}
	}
}
impl Drop for Importer {
	fn drop(&mut self) {
		unsafe { aiReleasePropertyStore(self.property_store) }
	}
}
