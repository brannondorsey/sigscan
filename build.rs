use std::env;
use std::fs;
use std::path::Path;

const EXAMPLES_SECTION_HEADER: &str = "## Examples";

fn main() {
    // Tell Cargo to re-run this build script if README.md changes
    println!("cargo:rerun-if-changed=README.md");

    // Read the README.md file
    let readme_content = fs::read_to_string("README.md").expect("Failed to read README.md");

    // Extract the examples section
    let failure_message = format!("Failed to extract {EXAMPLES_SECTION_HEADER} section");
    let examples_content = extract_examples_section(&readme_content).expect(&failure_message);

    // Create the output directory if it doesn't exist
    let out_dir = env::var_os("OUT_DIR").expect("Failed to get OUT_DIR");
    let dest_path = Path::new(&out_dir).join("examples_content.rs");

    // Write the content to a generated Rust file
    // Use raw string literals with a more unique delimiter to handle backticks in the content
    fs::write(
        &dest_path,
        format!(
            "pub const EXAMPLES_CONTENT: &str = r###\"{}\"###;",
            examples_content
        ),
    )
    .expect("Failed to write examples content to file");
}

fn extract_examples_section(readme_content: &str) -> Option<String> {
    // Look for the Examples section header
    if let Some(examples_start) = readme_content.find(EXAMPLES_SECTION_HEADER) {
        // Find the next section header after Examples (one that starts with a double # character)
        let examples_section = readme_content[examples_start..].to_string();
        let lines: Vec<&str> = examples_section.lines().collect();

        // Find the next top-level section header (# )
        if let Some(next_section_idx) = lines
            .iter()
            .skip(1)
            .position(|line| line.starts_with("## "))
        {
            // Get the lines up to the next section
            let examples_content = lines[..next_section_idx + 1].join("\n");
            return Some(examples_content);
        } else {
            // If there's no next section, return all lines
            return Some(examples_section);
        }
    }

    None
}
