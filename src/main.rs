#![allow(dead_code)]
pub mod codec;
pub mod generator;
pub mod model;
pub mod output;
pub mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;

    // Import needed modules
    use crate::output::{generate_common, generate_fix_files};
    use crate::parser::FixParser;

    // Use the existing src/output directory for generated files
    let output_dir = Path::new("src/output");

    // Ensure the output directory exists
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Generate common utilities
    match generate_common(output_dir) {
        Ok(_) => println!("Generated common utilities successfully"),
        Err(e) => eprintln!("Error generating common utilities: {}", e),
    }

    // Parse and generate code for each specification file
    let specs_dir = Path::new("src/specs");
    for entry in fs::read_dir(specs_dir)? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                continue;
            }
        };

        let path = entry.path();

        // Only process XML files
        if let Some(extension) = path.extension() {
            if extension == "xml" {
                println!("Processing: {}", path.display());

                // Parse the XML file and generate code
                match FixParser::parse_file(&path) {
                    Ok(fix_version) => match generate_fix_files(output_dir, &fix_version) {
                        Ok(_) => println!("Generated code for {} successfully", path.display()),
                        Err(e) => eprintln!("Error generating code for {}: {}", path.display(), e),
                    },
                    Err(e) => eprintln!("Error parsing {}: {}", path.display(), e),
                }
            }
        }
    }

    println!("Code generation completed!");
    Ok(())
}
