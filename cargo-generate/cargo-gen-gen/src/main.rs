use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let template_manifest_dir = manifest_dir.parent().unwrap().parent().unwrap();
    let template_manifest = template_manifest_dir.join("Cargo.toml");

    let cargo_toml = std::fs::read_to_string(&template_manifest).unwrap();
    let stable_liquid_contents = cargo_toml
        .lines()
        .map(|l| {
            if l.contains("name = \"trunk-template\"") {
                "name = \"{{project-name}}\""
            } else {
                l
            }
        })
        .collect::<Vec<&str>>()
        .join("\n");
    std::fs::write(
        manifest_dir.parent().unwrap().join("Cargo.toml.liquid"),
        &stable_liquid_contents,
    )
    .unwrap();

    println!("generating lock file for stable...");
    Command::new("/usr/bin/cargo")
        .arg("update")
        .current_dir(template_manifest_dir)
        .status()
        .unwrap();

    let lock_file = template_manifest_dir.join("Cargo.lock");
    let lock_file_contents = std::fs::read_to_string(&lock_file).unwrap();

    let replace_in_lock = |contents: &str| {
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

    let new_lock_file_contents = replace_in_lock(&lock_file_contents);

    let cargo_gen_dir = manifest_dir.parent().unwrap();

    std::fs::write(
        cargo_gen_dir.join("Cargo.lock.liquid"),
        new_lock_file_contents,
    )
    .unwrap();

    let stable_manifest_contents = std::fs::read_to_string(&template_manifest).unwrap();
    std::fs::rename(lock_file, template_manifest_dir.join("Cargo.lock.old")).unwrap();
    std::fs::rename(
        template_manifest_dir.join("Cargo.toml"),
        template_manifest_dir.join("Cargo.toml.old"),
    )
    .unwrap();

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

    let next_manifest_liquid_contents = replace_yew(&stable_liquid_contents);

    std::fs::write(
        cargo_gen_dir.join("Cargo.toml.next.liquid"),
        next_manifest_liquid_contents,
    )
    .unwrap();

    println!("generating lock file for next...");
    std::fs::write(
        template_manifest_dir.join("Cargo.toml"),
        replace_yew(&stable_manifest_contents),
    )
    .unwrap();

    Command::new("/usr/bin/cargo")
        .arg("update")
        .current_dir(template_manifest_dir)
        .status()
        .unwrap();
    let next_lock_contents =
        std::fs::read_to_string(template_manifest_dir.join("Cargo.lock")).unwrap();
    std::fs::write(
        cargo_gen_dir.join("Cargo.lock.next.liquid"),
        replace_in_lock(&next_lock_contents),
    )
    .unwrap();
    std::fs::rename(
        template_manifest_dir.join("Cargo.lock"),
        cargo_gen_dir.join("Cargo.lock.next.liquid"),
    )
    .unwrap();
    std::fs::rename(
        template_manifest_dir.join("Cargo.lock.old"),
        template_manifest_dir.join("Cargo.lock"),
    )
    .unwrap();
    std::fs::rename(
        template_manifest_dir.join("Cargo.toml.old"),
        template_manifest_dir.join("Cargo.toml"),
    )
    .unwrap();

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
}
