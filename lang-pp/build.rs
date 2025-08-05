use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Generate interned strings
    string_cache_codegen::AtomType::new("exts::names::ExtNameAtom", "ext_name!")
        .atoms(&[
            "GL_ARB_shading_language_include",
            "GL_GOOGLE_cpp_style_line_directive",
            "GL_GOOGLE_include_directive",
        ])
        .write_to_file(&out_dir.join("ext_names.rs"))
        .expect("failed to generate atoms");

    string_cache_codegen::AtomType::new("types::type_names::TypeNameAtom", "type_name!")
        .atoms(&[
            "void",
            "int",
            "bool",
            "float",
            "double",
            // HLSL type names
            "float2",
            "float3",
            "float4",
            "int2",
            "int3",
            "int4",
            "bool2",
            "bool3",
            "bool4",
            "uint",
            "uint2",
            "uint3",
            "uint4",
            // HLSL matrix types
            "float1x1",
            "float1x2",
            "float1x3",
            "float1x4",
            "float2x1",
            "float2x2",
            "float2x3",
            "float2x4",
            "float3x1",
            "float3x2",
            "float3x3",
            "float3x4",
            "float4x1",
            "float4x2",
            "float4x3",
            "float4x4",
            // Basic sampler type for HLSL
            "sampler",
        ])
        .write_to_file(&out_dir.join("type_names.rs"))
        .expect("failed to generate atoms");

    string_cache_codegen::AtomType::new("types::keywords::KeywordAtom", "keyword!")
        .atoms(&[
            "const",
            "uniform",
            "buffer",
            "shared",
            "attribute",
            "varying",
            "coherent",
            "volatile",
            "restrict",
            "readonly",
            "writeonly",
            "layout",
            "centroid",
            "flat",
            "smooth",
            "noperspective",
            "patch",
            "sample",
            "invariant",
            "precise",
            "break",
            "continue",
            "do",
            "for",
            "while",
            "switch",
            "case",
            "default",
            "if",
            "else",
            "subroutine",
            "in",
            "out",
            "inout",
            "true",
            "false",
            "discard",
            "return",
            "lowp",
            "mediump",
            "highp",
            "precision",
            "struct",
            "cbuffer",
            // Reserved for future use
            "common",
            "partition",
            "active",
            "asm",
            "class",
            "union",
            "enum",
            "typedef",
            "template",
            "this",
            "resource",
            "goto",
            "inline",
            "noinline",
            "public",
            "static",
            "extern",
            "external",
            "interface",
            "long",
            "short",
            "half",
            "fixed",
            "unsigned",
            "superp",
            "input",
            "output",
            "filter",
            "sizeof",
            "cast",
            "namespace",
            "using",
        ])
        .write_to_file(&out_dir.join("keywords.rs"))
        .expect("failed to generate atoms");
}
