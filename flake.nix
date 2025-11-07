{
	description = "nix build environment";
	
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
	};

	outputs = { self, nixpkgs, fenix }:
		let
			universal = function:
				nixpkgs.lib.genAttrs [
					"x86_64-linux"
					"aarch64-linux"
				] (system: function nixpkgs.legacyPackages.${system});

			overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
		in {
			devShell = universal (pkgs: 
				(pkgs.mkShell rec {
					name = "rust";

					nativeBuildInputs = with pkgs; [
						renderdoc

						#cargo
						#clippy
						#rustc
						#rustfmt
						rustup
						rustPlatform.bindgenHook

						pkg-config
					];

					buildInputs = with pkgs; [
						freetype
						libGL
						xorg.libX11
						xorg.libXrandr
						xorg.libXcursor
						xorg.libXi
						xorg.libXinerama
						xorg.libXtst
						xorg.libXxf86vm
						libxkbcommon
						udev
						glib
						fontconfig
						gtk3
						cairo
						pango
						harfbuzz
						atk
						gobject-introspection
						gdk-pixbuf
						glfw
						stdenv.cc.cc.lib
					];

					RUSTC_VERSION = overrides.toolchain.channel;
					#RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
					LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
					
					shellHook = ''
						export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
						export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${pkgs.stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
					'';
				}));
		};
}
