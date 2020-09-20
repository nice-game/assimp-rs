use bindgen::CargoCallbacks;
use std::{env::var, path::PathBuf};

fn main() {
	let no_export = if cfg!(feature = "export") { "OFF" } else { "ON" };
	let assimp_dir = cmake::Config::new("assimp")
		.define("BUILD_SHARED_LIBS", "OFF")
		.define("ASSIMP_NO_EXPORT", no_export)
		.define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
		.define("ASSIMP_BUILD_TESTS", "OFF")
		.define("INJECT_DEBUG_POSTFIX", "OFF")
		.define("ASSIMP_INSTALL_PDB", "OFF")
		// .define("CMAKE_SUPPRESS_DEVELOPER_WARNINGS", "ON")
		.define("LIBRARY_SUFFIX", "")
		.build();
	let debug_postfix = if var("DEBUG").unwrap() == "true" { "d" } else { "" };
	println!("cargo:rustc-link-search=native={}", assimp_dir.join("lib").display());
	println!("cargo:rustc-link-lib=static=assimp{}", debug_postfix);
	println!("cargo:rustc-link-lib=static=IrrXML{}", debug_postfix);

	println!("cargo:rerun-if-changed=wrapper.h");
	bindgen::Builder::default()
		.header("wrapper.h")
		.parse_callbacks(Box::new(CargoCallbacks))
		.clang_args(&["-I", assimp_dir.join("include").to_str().unwrap()])
		.clang_arg("-v")
		.whitelist_type("aiPostProcessSteps")
		.whitelist_function("aiCreatePropertyStore")
		.whitelist_function("aiGetErrorString")
		.whitelist_function("aiImportFileExWithProperties")
		.whitelist_function("aiReleaseImport")
		.whitelist_function("aiReleasePropertyStore")
		.generate()
		.unwrap()
		.write_to_file(PathBuf::from(var("OUT_DIR").unwrap()).join("bindings.rs"))
		.unwrap();
}
