switch variable::get("yew_version"){
  "stable" => {
    file::rename("cargo-generate/Cargo.lock.liquid", "Cargo.lock");
    file::rename("cargo-generate/Cargo.toml.liquid", "Cargo.toml");
  }
  "next" => {
    file::rename("cargo-generate/Cargo.lock.next.liquid", "Cargo.lock");
    file::rename("cargo-generate/Cargo.toml.next.liquid", "Cargo.toml");
  }
}
file::rename("cargo-generate/_README.md", "README.md");

file::delete("cargo-generate");
file::delete(".github");
