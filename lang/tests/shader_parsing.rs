use hlsl_lang::{ast, parse::DefaultParse};
use std::fs;
use std::path::Path;
use std::collections::HashSet;

/// Shaders that are expected to fail parsing
const EXPECTED_FAIL_SHADERS: &[&str] = &[
    "shaders/FxDis/test.hlsl",
    "shaders/HlslCrossCompiler/ps4/fxaa.hlsl",
    "shaders/HlslCrossCompiler/ps5/interface_arrays.hlsl",
    "shaders/HlslCrossCompiler/ps5/interfaces.hlsl",
    "shaders/HlslCrossCompiler/hs5/basic.hlsl",
    "shaders/HlslCrossCompiler/vs5/const_temp.hlsl",
    "shaders/HlslCrossCompiler/ds5/basic.hlsl",
    "shaders/Internal/Textures.hlsl",
    "shaders/Sdk/Direct3D11/ContactHardeningShadows11/ContactHardeningShadows11.hlsl",
    "shaders/Sdk/Direct3D11/SimpleBezier11/SimpleBezier11.hlsl",
    "shaders/Sdk/Direct3D11/DynamicShaderLinkage11/DynamicShaderLinkage11_PS.hlsl",
    "shaders/Sdk/Direct3D11/DynamicShaderLinkage11/DynamicShaderLinkage11_VS.hlsl",
    "shaders/Sdk/Direct3D11/DecalTessellation11/DecalTessellation11.hlsl",
    "shaders/Sdk/Direct3D11/PNTriangles11/PNTriangles11.hlsl",
    "shaders/Sdk/Direct3D11/SubD11/SubD11.hlsl",
    "shaders/Sdk/Direct3D11/CascadedShadowMaps11/RenderCascadeScene.hlsl",
    "shaders/Sdk/Direct3D11/CascadedShadowMaps11/RenderCascadeShadow.hlsl",
    "shaders/Sdk/Direct3D11/NBodyGravityCS11/NBodyGravityCS11.hlsl",
    "shaders/Sdk/Direct3D11/NBodyGravityCS11/ParticleDraw.hlsl",
    "shaders/Sdk/Direct3D11/FluidCS11/FluidRender.hlsl",
    "shaders/Sdk/Direct3D11/FluidCS11/FluidCS11.hlsl",
    "shaders/Sdk/Direct3D11/DDSWithoutD3DX11/DDSWithoutD3DX.hlsl",
    "shaders/Sdk/Direct3D11/BasicCompute11/BasicCompute11.hlsl",
    "shaders/Sdk/Direct3D11/DetailTessellation11/POM.hlsl",
    "shaders/Sdk/Direct3D11/DetailTessellation11/Particle.hlsl",
    "shaders/Sdk/Direct3D11/DetailTessellation11/DetailTessellation11.hlsl",
    "shaders/Sdk/Direct3D11/ComputeShaderSort11/ComputeShaderSort11.hlsl",
    "shaders/Sdk/Direct3D11/AdaptiveTessellationCS40/TessellatorCS40_TessellateVerticesCS.hlsl",
    "shaders/Sdk/Direct3D11/AdaptiveTessellationCS40/TessellatorCS40_common.hlsl",
    "shaders/Sdk/Direct3D11/AdaptiveTessellationCS40/TessellatorCS40_TessellateIndicesCS.hlsl",
    "shaders/Sdk/Direct3D11/AdaptiveTessellationCS40/TessellatorCS40_EdgeFactorCS.hlsl",
    "shaders/Sdk/Direct3D11/AdaptiveTessellationCS40/TessellatorCS40_NumVerticesIndicesCS.hlsl",
    "shaders/Sdk/Direct3D11/AdaptiveTessellationCS40/TessellatorCS40_ScatterIDCS.hlsl",
    "shaders/Sdk/Direct3D11/BC6HBC7EncoderDecoder11/BC7Encode.hlsl",
    "shaders/Sdk/Direct3D11/BC6HBC7EncoderDecoder11/BC6HDecode.hlsl",
    "shaders/Sdk/Direct3D11/BC6HBC7EncoderDecoder11/BC6HEncode.hlsl",
    "shaders/Sdk/Direct3D11/BC6HBC7EncoderDecoder11/BC7Decode.hlsl",
    "shaders/Sdk/Direct3D11/VarianceShadows11/RenderVarianceShadow.hlsl",
    "shaders/Sdk/Direct3D11/VarianceShadows11/2DQuadShaders.hlsl",
    "shaders/Sdk/Direct3D11/VarianceShadows11/RenderVarianceScene.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/FilterCS.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/BrightPassAndHorizFilterCS.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/skybox11.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/DumpToTexture.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/ReduceToSingleCS.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/PSApproach.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/FinalPass.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/ReduceTo1DCS.hlsl",
];

/// Test that all HLSL shaders in the shaders directory can be parsed successfully
/// except for those explicitly listed in EXPECTED_FAIL_SHADERS
#[test]
fn test_shader_parsing() {
    let shader_dir = Path::new("shaders");
    
    if !shader_dir.exists() {
        panic!("Shader directory not found: {:?}", shader_dir);
    }

    let expected_fail_set: HashSet<&str> = EXPECTED_FAIL_SHADERS.iter().copied().collect();

    let mut total_shaders = 0;
    let mut actual_pass_shaders = Vec::new();
    let mut actual_fail_shaders = Vec::new();

    // Walk through all .hlsl files in the shaders directory
    for entry in walkdir::WalkDir::new(shader_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "hlsl"))
    {
        let path = entry.path();
        let path_str = path.to_string_lossy();
        total_shaders += 1;

        // Read the shader file
        let source = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                actual_fail_shaders.push((path_str.to_string(), format!("Failed to read file - {}", e)));
                continue;
            }
        };

        // Try to parse the shader
        match ast::TranslationUnit::parse(&source) {
            Ok(_) => {
                println!("✓ Parsed successfully: {}", path_str);
                actual_pass_shaders.push(path_str.to_string());
            }
            Err(e) => {
                actual_fail_shaders.push((path_str.to_string(), format!("Parse error - {}", e)));
            }
        }
    }

    println!("\nShader parsing results:");
    println!("Total shaders tested: {}", total_shaders);
    println!("Successfully parsed: {}", actual_pass_shaders.len());
    println!("Failed to parse: {}", actual_fail_shaders.len());

    // Convert actual results to sets for comparison
    let actual_pass_set: HashSet<String> = actual_pass_shaders.into_iter().collect();
    let actual_fail_set: HashSet<String> = actual_fail_shaders.iter().map(|(path, _)| path.clone()).collect();

    // Check for unexpected passes (shaders that were expected to fail but passed)
    let unexpected_passes: Vec<_> = actual_pass_set
        .iter()
        .filter(|shader| expected_fail_set.contains(shader.as_str()))
        .collect();

    // Check for unexpected failures (shaders that were expected to pass but failed)
    let unexpected_failures: Vec<_> = actual_fail_set
        .iter()
        .filter(|shader| !expected_fail_set.contains(shader.as_str()))
        .collect();

    let mut test_failed = false;

    if !unexpected_passes.is_empty() {
        println!("\n❌ UNEXPECTED PASSES (shaders that were expected to fail but passed):");
        for shader in &unexpected_passes {
            println!("  {}", shader);
        }
        println!("Consider removing these from EXPECTED_FAIL_SHADERS");
        test_failed = true;
    }

    if !unexpected_failures.is_empty() {
        println!("\n❌ UNEXPECTED FAILURES (shaders that were expected to pass but failed):");
        for shader in &unexpected_failures {
            if let Some((_, error)) = actual_fail_shaders.iter().find(|(path, _)| path == *shader) {
                println!("  {}: {}", shader, error);
            }
        }
        println!("Consider adding these to EXPECTED_FAIL_SHADERS if the failures are expected");
        test_failed = true;
    }

    // Print errors for expected failures for debugging purposes
    let expected_failures: Vec<_> = actual_fail_set
        .iter()
        .filter(|shader| expected_fail_set.contains(shader.as_str()))
        .collect();

    if !expected_failures.is_empty() {
        println!("\n📋 EXPECTED FAILURES (parsing errors for reference):");
        for shader in &expected_failures {
            if let Some((_, error)) = actual_fail_shaders.iter().find(|(path, _)| path == *shader) {
                println!("  {}: {}", shader, error);
            }
        }
    }

    if test_failed {
        panic!("Shader parsing results did not match expectations. See output above for details.");
    }

    println!("\n✅ All shader parsing results matched expectations!");
    assert!(total_shaders > 0, "Should have found some shader files to test");
}