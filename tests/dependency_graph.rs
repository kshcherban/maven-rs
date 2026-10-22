// ABOUTME: Checks which Maven feature combinations require XML editing.
// ABOUTME: Protects parsing and resolver consumers from the optional editor dependency.

use std::process::Command;

#[test]
fn xml_editor_is_only_required_by_editor_feature() -> Result<(), Box<dyn std::error::Error>> {
    for features in ["", "resolver", "editor", "editor,resolver"] {
        let output = Command::new(env!("CARGO"))
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args([
                "tree",
                "--locked",
                "--no-default-features",
                "--features",
                features,
                "--edges",
                "normal,build",
                "--prefix",
                "none",
                "--format",
                "{p}",
            ])
            .output()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let graph = String::from_utf8(output.stdout)?;
        let requires_editor = graph
            .lines()
            .any(|package| package.starts_with("edit-xml "));
        assert_eq!(
            requires_editor,
            features.contains("editor"),
            "Features: {features}\n{graph}"
        );
    }
    Ok(())
}
