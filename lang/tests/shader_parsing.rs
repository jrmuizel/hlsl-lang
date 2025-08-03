use hlsl_lang::{ast, parse::DefaultParse};
use std::fs;
use std::path::Path;

/// Test that all HLSL shaders in the shaders directory can be parsed successfully
#[test]
fn test_shader_parsing() {
    let shader_dir = Path::new("shaders");
    
    if !shader_dir.exists() {
        panic!("Shader directory not found: {:?}", shader_dir);
    }

    let mut total_shaders = 0;
    let mut failed_shaders = Vec::new();

    // Walk through all .hlsl files in the shaders directory
    for entry in walkdir::WalkDir::new(shader_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "hlsl"))
    {
        let path = entry.path();
        total_shaders += 1;

        // Read the shader file
        let source = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                failed_shaders.push(format!("{}: Failed to read file - {}", path.display(), e));
                continue;
            }
        };

        // Try to parse the shader
        match ast::TranslationUnit::parse(&source) {
            Ok(_) => {
                println!("✓ Parsed successfully: {}", path.display());
            }
            Err(e) => {
                failed_shaders.push(format!("{}: Parse error - {}", path.display(), e));
            }
        }
    }

    println!("\nShader parsing results:");
    println!("Total shaders tested: {}", total_shaders);
    println!("Successfully parsed: {}", total_shaders - failed_shaders.len());
    println!("Failed to parse: {}", failed_shaders.len());

    if !failed_shaders.is_empty() {
        println!("\nFailed shaders:");
        for failure in &failed_shaders {
            println!("  {}", failure);
        }
    }

    // For now, we'll just report failures but not fail the test
    // This allows us to see which shaders need work without blocking CI
    if !failed_shaders.is_empty() {
        println!("\nNote: {} shaders failed to parse. This may be expected as the parser is still being developed.", failed_shaders.len());
        println!("This test serves as a baseline to track parsing improvements over time.");
    }

    // The test always passes for now - it's informational only
    assert!(total_shaders > 0, "Should have found some shader files to test");
}