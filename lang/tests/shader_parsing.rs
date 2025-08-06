use hlsl_lang::{ast, parse::DefaultParse};
use std::fs;
use std::path::Path;
use std::collections::HashSet;
use std::panic;

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
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/FinalPass.hlsl",
    "shaders/Sdk/Direct3D11/HDRToneMappingCS11/ReduceTo1DCS.hlsl",
];

/// HLSL shaders from data directory that are expected to fail parsing
const EXPECTED_FAIL_DATA_HLSL: &[&str] = &[
    "hlsl.flattenOpaque.frag",
    "hlsl.intrinsics.comp",
    "hlsl.hull.2.tesc",
    "hlsl.PointSize.geom",
    "hlsl.namespace.frag",
    "hlsl.intrinsics.negative.comp",
    "hlsl.structbuffer.append.fn.frag",
    "hlsl.reflection.vert",
    "hlsl.domain.2.tese",
    "hlsl.rw.register.frag",
    "hlsl.memberFunCall.frag",
    "hlsl.buffer.frag",
    "hlsl.type.identifier.frag",
    "hlsl.matType.frag",
    "hlsl.samplecmp.dualmode.frag",
    "hlsl.promotions.frag",
    "hlsl.explicitDescriptorSet.frag",
    "hlsl.flattenSubset.frag",
    "hlsl.samplelevel.basic.dx10.vert",
    "hlsl.gather.basic.dx10.vert",
    "hlsl.tristream-append.geom",
    "hlsl.hull.ctrlpt-2.tesc",
    "hlsl.structbuffer.byte.frag",
    "hlsl.promote.vec1.frag",
    "hlsl.noSemantic.functionality1.comp",
    "hlsl.samplecmplevelzero.offset.dx10.frag",
    "hlsl.int.dot.frag",
    "hlsl.rw.bracket.frag",
    "hlsl.templatetypes.frag",
    "hlsl.basic.geom",
    "hlsl.init.frag",
    "hlsl.shift.per-set.frag",
    "hlsl.rw.swizzle.frag",
    "hlsl.getdimensions.rw.dx10.frag",
    "hlsl.tx.overload.frag",
    "hlsl.struct.frag",
    "hlsl.load.buffer.float.dx10.frag",
    "hlsl.rw.scalar.bracket.frag",
    "hlsl.samplecmp.negative2.frag",
    "hlsl.switch.frag",
    "hlsl.matpack-1.frag",
    "hlsl.texture.struct.frag",
    "hlsl.wavebroadcast.comp",
    "hlsl.this.frag",
    "hlsl.structbuffer.floatidx.comp",
    "hlsl.domain.3.tese",
    "hlsl.structin.vert",
    "hlsl.samplegrad.array.dx10.frag",
    "hlsl.samplebias.offsetarray.dx10.frag",
    "hlsl.hull.void.tesc",
    "hlsl.hull.3.tesc",
    "hlsl.gathercmpRGBA.basic.dx10.frag",
    "hlsl.load.rwtexture.array.dx10.frag",
    "hlsl.samplegrad.basic.dx10.frag",
    "hlsl.whileLoop.frag",
    "hlsl.gathercmpRGBA.array.dx10.frag",
    "hlsl.imagefetch-subvec4.comp",
    "hlsl.intrinsics.f1632.frag",
    "hlsl.snorm.uav.comp",
    "hlsl.gathercmpRGBA.offset.dx10.frag",
    "hlsl.include.vert",
    "hlsl.semantic.geom",
    "hlsl.load.rwbuffer.dx10.frag",
    "hlsl.hull.4.tesc",
    "hlsl.load.offsetarray.dx10.frag",
    "hlsl.specConstant.frag",
    "hlsl.partialInit.frag",
    "hlsl.intrinsics.promote.outputs.frag",
    "hlsl.sample.dx9.vert",
    "hlsl.pp.line.frag",
    "hlsl.matpack-pragma.frag",
    "hlsl.getdimensions.dx10.frag",
    "hlsl.samplecmplevelzero.offsetarray.dx10.frag",
    "hlsl.intrinsics.negative.vert",
    "hlsl.preprocessor.frag",
    "hlsl.loopattr.frag",
    "hlsl.calculatelod.dx10.frag",
    "hlsl.intrinsics.vert",
    "hlsl.load.2dms.dx10.frag",
    "hlsl.charLit.vert",
    "hlsl.hlslOffset.vert",
    "hlsl.numericsuffixes.frag",
    "hlsl.tx.bracket.frag",
    "hlsl.structbuffer.rwbyte.frag",
    "hlsl.wavereduction.comp",
    "hlsl.attribute.expression.comp",
    "hlsl.hull.5.tesc",
    "hlsl.samplecmp.offset.dx10.frag",
    "hlsl.doLoop.frag",
    "hlsl.texture.subvec4.frag",
    "hlsl.nonstaticMemberFunction.frag",
    "hlsl.earlydepthstencil.frag",
    "hlsl.params.default.negative.frag",
    "hlsl.load.buffer.dx10.frag",
    "hlsl.logicalConvert.frag",
    "hlsl.promote.atomic.frag",
    "hlsl.load.basic.dx10.vert",
    "hlsl.color.hull.tesc",
    "hlsl.clipdistance-1.geom",
    "hlsl.-D-U.frag",
    "hlsl.float4.frag",
    "hlsl.pp.line4.frag",
    "hlsl.constructimat.frag",
    "hlsl.samplecmp.offsetarray.dx10.frag",
    "hlsl.clipdistance-2.geom",
    "hlsl.texturebuffer.frag",
    "hlsl.sample.sub-vec4.dx10.frag",
    "hlsl.stringtoken.frag",
    "hlsl.constructArray.vert",
    "hlsl.samplelevel.offsetarray.dx10.frag",
    "hlsl.sample.basic.dx10.frag",
    "hlsl.attribute.frag",
    "hlsl.PointSize.vert",
    "hlsl.layout.frag",
    "hlsl.flattenOpaqueInit.vert",
    "hlsl.singleArgIntPromo.vert",
    "hlsl.typedef.frag",
    "hlsl.groupid.comp",
    "hlsl.cast.frag",
    "hlsl.structbuffer.fn2.comp",
    "hlsl.calculatelodunclamped.dx10.frag",
    "hlsl.structStructName.frag",
    "hlsl.load.offset.dx10.frag",
    "hlsl.self_cast.frag",
    "hlsl.matType.int.frag",
    "hlsl.gather.offsetarray.dx10.frag",
    "hlsl.hull.6.tesc",
    "hlsl.gathercmpRGBA.offsetarray.dx10.frag",
    "hlsl.samplegrad.offsetarray.dx10.frag",
    "hlsl.sample.offset.dx10.frag",
    "hlsl.gatherRGBA.array.dx10.frag",
    "hlsl.automap.frag",
    "hlsl.staticMemberFunction.frag",
    "hlsl.structIoFourWay.frag",
    "hlsl.getsampleposition.dx10.frag",
    "hlsl.reflection.binding.frag",
    "hlsl.scalar2matrix.frag",
    "hlsl.subpass.frag",
    "hlsl.samplegrad.offset.dx10.frag",
    "hlsl.forLoop.frag",
    "hlsl.samplebias.array.dx10.frag",
    "hlsl.shapeConv.frag",
    "hlsl.inf.vert",
    "hlsl.gather.basic.dx10.frag",
    "hlsl.samplelevel.offset.dx10.frag",
    "hlsl.samplelevel.basic.dx10.frag",
    "hlsl.rw.vec2.bracket.frag",
    "hlsl.gatherRGBA.offset.dx10.frag",
    "hlsl.samplecmp.array.dx10.frag",
    "hlsl.wavevote.comp",
    "hlsl.intrinsics.double.frag",
    "hlsl.pp.expand.frag",
    "hlsl.fraggeom.frag",
    "hlsl.load.array.dx10.frag",
    "hlsl.samplebias.offset.dx10.frag",
    "hlsl.gatherRGBA.offsetarray.dx10.frag",
    "hlsl.clipdistance-3.geom",
    "hlsl.samplecmplevelzero.array.dx10.frag",
    "hlsl.struct.split.trivial.geom",
    "hlsl.structbuffer.append.frag",
    "hlsl.structbuffer.atomics.frag",
    "hlsl.params.default.frag",
    "hlsl.string.frag",
    "hlsl.cbuffer-identifier.vert",
    "hlsl.gs-hs-mix.tesc",
    "hlsl.store.rwbyteaddressbuffer.type.comp",
    "hlsl.overload.frag",
    "hlsl.implicitBool.frag",
    "hlsl.multiEntry.vert",
    "hlsl.samplecmplevelzero.basic.dx10.frag",
    "hlsl.struct.split.nested.geom",
    "hlsl.mintypes.frag",
    "hlsl.sample.offsetarray.dx10.frag",
    "hlsl.wavequad.comp",
    "hlsl.structarray.flatten.geom",
    "hlsl.matType.bool.frag",
    "hlsl.autosampledtextures.frag",
    "hlsl.attributeC11.frag",
    "hlsl.load.basic.dx10.frag",
    "hlsl.templatetypes.negative.frag",
    "hlsl.flattenOpaqueInitMix.vert",
    "hlsl.gather.offset.dx10.frag",
    "hlsl.intrinsics.f3216.frag",
    "hlsl.samplecmp.basic.dx10.frag",
    "hlsl.samplelevel.array.dx10.frag",
    "hlsl.pp.line2.frag",
    "hlsl.gather.array.dx10.frag",
    "hlsl.load.rwtexture.dx10.frag",
    "hlsl.rw.atomics.frag",
    "hlsl.waveprefix.comp",
    "hlsl.samplebias.basic.dx10.frag",
    "hlsl.clipdistance-4.geom",
    "hlsl.dashI.vert",
    "hlsl.array.flatten.frag",
    "hlsl.type.half.frag",
    "hlsl.intrinsics.negative.frag",
    "hlsl.gatherRGBA.basic.dx10.frag",
    "hlsl.intrinsics.frag",
    "hlsl.float1.frag",
    "hlsl.sample.dx9.frag",
    "hlsl.multiDescriptorSet.frag",
    "hlsl.struct.split.array.geom",
    "hlsl.typeGraphCopy.vert",
    "hlsl.getdimensions.dx10.vert",
    "hlsl.identifier.sample.frag",
    "hlsl.constantbuffer.frag",
    "hlsl.attributeGlobalBuffer.frag",
    "hlsl.printf.comp",
    "hlsl.intrinsics.promote.frag",
    "hlsl.pp.line3.frag",
    "hlsl.hull.ctrlpt-1.tesc",
    "hlsl.samplecmp.negative.frag",
    "hlsl.structbuffer.coherent.frag",
    "hlsl.format.rwtexture.frag",
    "hlsl.hull.1.tesc",
    "hlsl.includeNegative.vert",
    "hlsl.sample.array.dx10.frag",
    "hlsl.domain.1.tese",
    "hlsl.if.frag",
    "hlsl.samplegrad.basic.dx10.vert",
    "hlsl.scalarCast.vert",
    "hlsl.layoutOverride.vert",
    "hlsl.structarray.flatten.frag",
    "hlsl.aliasOpaque.frag",
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

/// Test parsing HLSL shaders from the data directory
/// except for those explicitly listed in EXPECTED_FAIL_DATA_HLSL
#[test]
fn test_data_hlsl_parsing() {
    let data_dir = Path::new("../data");
    
    if !data_dir.exists() {
        panic!("Data directory not found: {:?}", data_dir);
    }

    let expected_fail_set: HashSet<&str> = EXPECTED_FAIL_DATA_HLSL.iter().copied().collect();

    let mut total_shaders = 0;
    let mut actual_pass_shaders = Vec::new();
    let mut actual_fail_shaders = Vec::new();

    // Find all files in data directory that start with "hlsl." 
    for entry in fs::read_dir(data_dir).expect("Failed to read data directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.starts_with("hlsl.") {
                total_shaders += 1;

                // Read the shader file
                let source = match fs::read_to_string(&path) {
                    Ok(content) => content,
                    Err(e) => {
                        actual_fail_shaders.push((file_name.to_string(), format!("Failed to read file - {}", e)));
                        continue;
                    }
                };

                // Try to parse the shader
                match ast::TranslationUnit::parse(&source) {
                    Ok(_) => {
                        actual_pass_shaders.push(file_name.to_string());
                    }
                    Err(_) => {
                        actual_fail_shaders.push((file_name.to_string(), "Parse error".to_string()));
                    }
                }
            }
        }
    }

    println!("\nData HLSL parsing results:");
    println!("Total HLSL shaders tested: {}", total_shaders);
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
        println!("Consider removing these from EXPECTED_FAIL_DATA_HLSL");
        test_failed = true;
    }

    if !unexpected_failures.is_empty() {
        println!("\n❌ UNEXPECTED FAILURES (shaders that were expected to pass but failed):");
        for shader in &unexpected_failures {
            if let Some((_, error)) = actual_fail_shaders.iter().find(|(path, _)| path == *shader) {
                println!("  {}: {}", shader, error);
            }
        }
        println!("Consider adding these to EXPECTED_FAIL_DATA_HLSL if the failures are expected");
        test_failed = true;
    }

    // Print errors for expected failures for debugging purposes
    let expected_failures: Vec<_> = actual_fail_set
        .iter()
        .filter(|shader| expected_fail_set.contains(shader.as_str()))
        .collect();

    if !expected_failures.is_empty() {
        println!("\n📋 EXPECTED FAILURES (parsing errors for reference):");
        for shader in expected_failures.iter().take(10) { // Limit to first 10 for brevity
            if let Some((_, error)) = actual_fail_shaders.iter().find(|(path, _)| path == *shader) {
                println!("  {}: {}", shader, error);
            }
        }
        if expected_failures.len() > 10 {
            println!("  ... and {} more expected failures", expected_failures.len() - 10);
        }
    }

    if test_failed {
        panic!("Data HLSL parsing results did not match expectations. See output above for details.");
    }

    println!("\n✅ All data HLSL parsing results matched expectations!");
    assert!(total_shaders > 0, "Should have found some HLSL shader files in data directory");
}
