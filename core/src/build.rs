use std::{env, fs, path::{Path, PathBuf}, str::FromStr};

pub fn link_assets<P: AsRef<Path>>(assets_directory: P, target_dir: Option<P>) {
	let target_dir = target_dir.map_or_else(|| {
		PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap())
	}, |v| {
		v.as_ref().to_path_buf()
	});

	let source = Path::new(env!("CARGO_MANIFEST_DIR")).join(&assets_directory);
	let dest = Path::new(&target_dir).join("assets");

	if dest.exists() {
		symlink::remove_symlink_dir(&dest).unwrap();
	}

	symlink::symlink_dir(&source, &dest).unwrap();

	println!("cargo:rerun-if-changed={}", assets_directory.as_ref().display());
}

pub fn link_test_assets() {
	link_assets(
		"tests/assets",
		PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap().join("../target/debug/deps").to_str()
	);
}
