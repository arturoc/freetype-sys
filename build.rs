#![feature(path_ext)]
extern crate pkg_config;

use std::process::Command;
use std::env;
use std::fs::{self,PathExt};
use std::path::Path;

#[cfg(unix)]
fn main() {
    match pkg_config::find_library("freetype2") {
        Ok(_) => return,
        Err(_) => {
			let freetype_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
			let freetype_native_dir = Path::new(&freetype_dir).join("freetype-2.5.5");
            if !freetype_native_dir.exists() {
				Command::new("curl")
					.current_dir(&freetype_dir)
					.arg("-L")
					.arg("http://download.savannah.gnu.org/releases/freetype/freetype-2.5.5.tar.gz")
					.arg("-O")
					.status().unwrap();
				Command::new("tar")
					.current_dir(&freetype_dir)
					.arg("xzf")
					.arg("freetype-2.5.5.tar.gz")
					.status().unwrap();
            }	
			Command::new("./configure")
				.current_dir(&freetype_native_dir)
				.status().unwrap();
			Command::new("make")
				.current_dir(&freetype_native_dir)
				.status().unwrap();
			let out_dir = env::var("OUT_DIR").unwrap();
			let dest_path = Path::new(&out_dir).join("libfreetype.a");
			fs::copy(freetype_native_dir.join("objs/.libs/libfreetype.a"),dest_path).unwrap();
			println!("cargo:rustc-flags= -L native={}",out_dir);
		}
    }
}
