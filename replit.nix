{ pkgs }: {
	deps = [
		pkgs.rustup
		pkgs.nano
  	pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}