switch variable::get("yew_version"){
  "stable" => {
    file::rename("cargo-generate/Cargo.lock", "Cargo.lock");
    file::rename("cargo-generate/Cargo.toml", "Cargo.toml");
  }
  "next" => {
    file::rename("cargo-generate/Cargo.lock.next", "Cargo.lock");
    file::rename("cargo-generate/Cargo.toml.next", "Cargo.toml");
  }
}
file::rename("cargo-generate/README.md", "README.md");

file::delete("cargo-generate");
file::delete(".github");
