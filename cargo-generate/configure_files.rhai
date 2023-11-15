file::delete("Cargo.toml");
file::rename("cargo-generate/Cargo.toml", "Cargo.toml");
file::delete("cargo-generate");
file::delete(".github");
