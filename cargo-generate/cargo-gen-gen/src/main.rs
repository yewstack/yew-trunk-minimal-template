use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template_manifest_dir = manifest_dir.parent().unwrap().parent().unwrap();
    let template_manifest = template_manifest_dir.join("Cargo.toml");
    let cargo_gen_dir = manifest_dir.parent().unwrap();

    // Helper function to replace project name in files
    let replace_project_name = |contents: &str| {
        contents
            .lines()
            .map(|l| {
                if l.contains("name = \"trunk-template\"") {
                    "name = \"{{project-name}}\""
                } else {
                    l
                }
            })
            .collect::<Vec<&str>>()
            .join("\n")
    };

    // Helper function to replace yew dependency
    let replace_yew = |contents: &str| {
        contents
            .lines()
            .map(|l| {
                if l.starts_with("yew =") {
                    "yew = { git = \"https://github.com/yewstack/yew\", branch = \"master\", features = [\"csr\"] }"
                } else {
                    l
                }
            })
            .collect::<Vec<&str>>()
            .join("\n")
    };

    // Helper function to add tracing dependencies
    let add_tracing_deps = |contents: &str| {
        contents
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .into_iter()
            .flat_map(|l| {
                if l.starts_with("yew =") {
                    vec![
                        l,
                        "tracing = \"0.1\"".to_string(),
                        "tracing-subscriber = { version = \"0.3\", features = [\"fmt\"] }".to_string(),
                        "tracing-web = \"0.1\"".to_string(),
                    ]
                } else {
                    vec![l]
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    };

    // Save original Cargo.toml
    let original_cargo_toml = std::fs::read_to_string(&template_manifest).unwrap();
    
    // Back up original files
    std::fs::rename(
        template_manifest_dir.join("Cargo.toml"),
        template_manifest_dir.join("Cargo.toml.orig"),
    ).unwrap();
    std::fs::rename(
        template_manifest_dir.join("Cargo.lock"),
        template_manifest_dir.join("Cargo.lock.orig"),
    ).unwrap_or({});

    // Generate 4 combinations of Cargo.toml.liquid and Cargo.lock.liquid files
    let configs = [
        ("stable", false),
        ("stable", true),
        ("next", false),
        ("next", true),
    ];

    for (yew_version, with_tracing) in configs.iter() {
        let suffix = format!(
            ".{}.{}",
            yew_version,
            if *with_tracing { "tracing" } else { "no-tracing" }
        );
        
        println!("Generating config for: yew={}, tracing={}", yew_version, with_tracing);

        // Create the appropriate Cargo.toml
        let mut cargo_toml_content = if *yew_version == "next" {
            replace_yew(&original_cargo_toml)
        } else {
            original_cargo_toml.clone()
        };

        if *with_tracing {
            cargo_toml_content = add_tracing_deps(&cargo_toml_content);
        }

        // Write temporary Cargo.toml
        std::fs::write(&template_manifest, &cargo_toml_content).unwrap();

        // Generate lock file
        println!("  Generating lock file...");
        Command::new("cargo")
            .arg("update")
            .arg("--quiet")
            .current_dir(template_manifest_dir)
            .status()
            .unwrap();

        // Read and process lock file
        let lock_file = template_manifest_dir.join("Cargo.lock");
        let lock_contents = std::fs::read_to_string(&lock_file).unwrap();
        let lock_liquid = replace_project_name(&lock_contents);

        // Save liquid templates
        let cargo_toml_liquid = replace_project_name(&cargo_toml_content);
        std::fs::write(
            cargo_gen_dir.join(format!("Cargo.toml{}.liquid", suffix)),
            cargo_toml_liquid,
        ).unwrap();

        std::fs::write(
            cargo_gen_dir.join(format!("Cargo.lock{}.liquid", suffix)),
            lock_liquid,
        ).unwrap();
    }

    // Restore original files
    std::fs::rename(
        template_manifest_dir.join("Cargo.toml.orig"),
        template_manifest_dir.join("Cargo.toml"),
    ).unwrap();
    std::fs::rename(
        template_manifest_dir.join("Cargo.lock.orig"),
        template_manifest_dir.join("Cargo.lock"),
    ).unwrap_or({});

    // Process README
    let md = std::fs::read_to_string(template_manifest_dir.join("README.md")).unwrap();
    let update_instruction = "Update the `name`,";
    assert_eq!(md.match_indices(update_instruction).count(), 1);
    let new_md = md.replace(update_instruction, "Update the");
    let if_you_cloned = "If you cloned";
    assert_eq!(new_md.match_indices(if_you_cloned).count(), 1);
    let new_md = new_md
        .lines()
        .filter(|l| !l.starts_with(if_you_cloned))
        .collect::<Vec<&str>>()
        .join("\n");

    std::fs::write(cargo_gen_dir.join("_README.md"), new_md).unwrap();
    
    println!("Successfully generated 4 template combinations!");
}
