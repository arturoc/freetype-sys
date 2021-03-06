extern crate pkg_config;
extern crate cmake;

use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

static VERSION: &str = "2.8.1";

fn build_unix() {
    match pkg_config::find_library("freetype2") {
        Ok(_) => return,
        Err(_) => {
			let freetype_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
			let freetype_native_dir = Path::new(&freetype_dir).join(&format!("freetype-{}", VERSION));
            fs::create_dir(freetype_native_dir.join("objs")).is_ok();
			#[cfg(unix)]
			fs::set_permissions(freetype_native_dir.join("configure"), fs::Permissions::from_mode(0o755))
				.unwrap();
			Command::new("./configure")
				.current_dir(&freetype_native_dir)
				.arg("--without-bzip2")
				.arg("--with-harfbuzz=no")
				.arg("--enable-static=yes")
				.arg("--enable-shared=no")
				.arg("--with-zlib=no")
				.status().unwrap();
			Command::new("make")
				.current_dir(&freetype_native_dir)
				.status().unwrap();
			let out_dir = env::var("OUT_DIR").unwrap();
			let dest_path = Path::new(&out_dir).join("libfreetype.a");
			fs::copy(freetype_native_dir.join("objs/.libs/libfreetype.a"),dest_path).unwrap();
			println!("cargo:rustc-link-search=native={}",out_dir);
			println!("cargo:link_search={}",out_dir);
			println!("cargo:include_search={}",freetype_native_dir.join("include").to_str().unwrap());
		}
    }
}

fn build_windows() {
	let dst = cmake::build("freetype-2.8.1");
	let lib_dir = dst.join("lib");
	println!("cargo:rustc-link-search=native={}", lib_dir.display());

	#[cfg(debug_assertions)]
	println!("cargo:rustc-link-lib=static=freetyped");

	#[cfg(not(debug_assertions))]
	println!("cargo:rustc-link-lib=static=freetype");

}

fn build_emscripten() {
    match pkg_config::find_library("freetype2") {
        Ok(_) => return,
        Err(_) => {
			let freetype_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
			let freetype_native_dir = Path::new(&freetype_dir).join(&format!("freetype-{}", VERSION));
            fs::create_dir(freetype_native_dir.join("objs")).is_ok();
			#[cfg(unix)]
			fs::set_permissions(freetype_native_dir.join("configure"), fs::Permissions::from_mode(0o755))
				.unwrap();
			Command::new("./configure")
				.current_dir(&freetype_native_dir)
				.arg("--without-bzip2")
				.arg("--with-harfbuzz=no")
				.arg("--enable-static=yes")
				.arg("--enable-shared=no")
				.arg("--with-zlib=no")
                .arg("--with-png=no")
				.status()
				.unwrap();

            Command::new("make")
                .arg("clean")
				.current_dir(&freetype_native_dir)
				.status().unwrap();
			Command::new("make")
				.current_dir(&freetype_native_dir)
				.status().unwrap();
            fs::copy(freetype_native_dir.join("objs").join("apinames"), freetype_native_dir.join("apinames")).unwrap();

			Command::new("emconfigure")
				.current_dir(&freetype_native_dir)
                .arg("./configure")
				.arg("--without-bzip2")
				.arg("--with-harfbuzz=no")
				.arg("--enable-static=yes")
				.arg("--enable-shared=no")
				.arg("--with-zlib=no")
                .arg("--with-png=no")
				.status().unwrap();
			Command::new("emmake")
                .arg("make")
                .arg("clean")
				.current_dir(&freetype_native_dir)
				.status().unwrap();
            fs::copy(freetype_native_dir.join("apinames"), freetype_native_dir.join("objs").join("apinames")).unwrap();
			Command::new("emmake")
                .arg("make")
				.current_dir(&freetype_native_dir)
				.status().unwrap();

			let out_dir = env::var("OUT_DIR").unwrap();
			let dest_path = Path::new(&out_dir).join("libfreetype.a");
			fs::copy(freetype_native_dir.join("objs/.libs/libfreetype.a"),dest_path).unwrap();
			println!("cargo:rustc-link-search=native={}",out_dir);
			println!("cargo:link_search={}",out_dir);
			println!("cargo:include_search={}",freetype_native_dir.join("include").to_str().unwrap());
		}
    }
}


fn main(){
	let target_triple = env::var("TARGET").unwrap();
	if target_triple.contains("linux") {
		build_unix()
	}else if target_triple.contains("darwin") {
		build_unix()
	}else if target_triple.contains("windows") {
		build_windows()
	}else if target_triple.contains("emscripten") {
		build_emscripten()
	}else if target_triple.contains("wasm32"){
		build_emscripten()
	}else{
		panic!("target OS {} not suported yet", target_triple);
	}
}
