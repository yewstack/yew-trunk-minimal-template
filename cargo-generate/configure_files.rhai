let yew_version = variable::get("yew_version");
let enable_tracing = variable::get("enable_tracing");

let tracing_suffix = if enable_tracing == "yes" { "tracing" } else { "no-tracing" };
let config_suffix = yew_version + "." + tracing_suffix;

// Rename the appropriate Cargo files based on yew version and tracing choice
file::rename("cargo-generate/Cargo.toml." + config_suffix, "Cargo.toml");
file::rename("cargo-generate/Cargo.lock." + config_suffix, "Cargo.lock");

// Rename the appropriate main.rs file based on tracing choice
file::rename("cargo-generate/main." + tracing_suffix + ".rs", "src/main.rs");

file::rename("cargo-generate/_README.md", "README.md");

file::delete("cargo-generate");
file::delete(".github");
