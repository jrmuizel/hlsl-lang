use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::types::type_names::TypeNameAtom;

#[macro_use]
pub mod names;

use names::ExtNameAtom;

pub struct ExtensionSpec {
    name: ExtNameAtom,
    type_names: Vec<TypeNameAtom>,
}

impl ExtensionSpec {
    pub fn new(name: ExtNameAtom, type_names: Vec<TypeNameAtom>) -> Self {
        Self { name, type_names }
    }

    pub fn name(&self) -> &ExtNameAtom {
        &self.name
    }

    pub fn type_names(&self) -> &[TypeNameAtom] {
        &self.type_names
    }
}

pub struct Registry {
    extensions: HashMap<ExtNameAtom, ExtensionSpec>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            extensions: Default::default(),
        }
    }

    pub fn all(&self) -> impl Iterator<Item = &ExtensionSpec> {
        self.extensions.values()
    }

    pub fn get(&self, name: &ExtNameAtom) -> Option<&ExtensionSpec> {
        self.extensions.get(name)
    }

    pub fn add(&mut self, spec: ExtensionSpec) {
        self.extensions.insert(spec.name.clone(), spec);
    }
}

// TODO: Fill registry with extension data from Khronos

impl Default for Registry {
    fn default() -> Self {
        Self {
            extensions: [
                ExtensionSpec::new(ExtNameAtom::from("GL_3DL_array_objects"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_AMD_gpu_shader_int16"),
                    vec![
                        TypeNameAtom::from("int16_t"),
                        TypeNameAtom::from("i16vec2"),
                        TypeNameAtom::from("i16vec3"),
                        TypeNameAtom::from("i16vec4"),
                        TypeNameAtom::from("uint16_t"),
                        TypeNameAtom::from("u16vec2"),
                        TypeNameAtom::from("u16vec3"),
                        TypeNameAtom::from("u16vec4"),
                    ],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_compute_shader"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_derivative_control"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_enhanced_layouts"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_explicit_attrib_location"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_explicit_uniform_location"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_gpu_shader5"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_gpu_shader_fp64"),
                    vec![
                        type_name!("double"),
                        // GLSL-specific double vector and matrix types removed for HLSL
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_gpu_shader_int64"),
                    vec![
                        TypeNameAtom::from("int64_t"),
                        TypeNameAtom::from("i64vec2"),
                        TypeNameAtom::from("i64vec3"),
                        TypeNameAtom::from("i64vec4"),
                        TypeNameAtom::from("uint64_t"),
                        TypeNameAtom::from("u64vec2"),
                        TypeNameAtom::from("u64vec3"),
                        TypeNameAtom::from("u64vec4"),
                    ],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_sample_shading"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_separate_shader_objects"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_shader_bit_encoding"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_shader_draw_parameters"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_shader_image_load_store"),
                    vec![
                        // GLSL-specific image types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_shader_image_size"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_shader_storage_buffer_object"),
                    vec![],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_shader_texture_image_samples"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_shader_texture_lod"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_shading_language_420pack"), vec![]),
                ExtensionSpec::new(ext_name!("GL_ARB_shading_language_include"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_shading_language_packing"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_sparse_texture2"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_tessellation_shader"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_texture_cube_map_array"),
                    vec![
                        // GLSL-specific sampler types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_texture_gather"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_texture_multisample"),
                    vec![
                        // GLSL-specific multisample sampler types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_texture_query_lod"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_ARB_texture_rectangle"),
                    vec![
                        // GLSL-specific rectangle sampler types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_uniform_buffer_object"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_vertex_attrib_64bit"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_ARB_viewport_array"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_YUV_target"),
                    vec![TypeNameAtom::from("__samplerExternal2DY2YEXT")],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_blend_func_extended"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_control_flow_attributes"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_device_group"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_frag_depth"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_geometry_shader"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_gpu_shader5"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_multiview"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_nonuniform_qualifier"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_null_initializer"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_primitive_bounding_box"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_ray_flags_primitive_culling"),
                    vec![],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_ray_query"),
                    vec![
                        TypeNameAtom::from("accelerationStructureEXT"),
                        TypeNameAtom::from("rayQueryEXT"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_samplerless_texture_functions"),
                    vec![],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_explicit_arithmetic_types"),
                    vec![
                        TypeNameAtom::from("float64_t"),
                        TypeNameAtom::from("f64vec2"),
                        TypeNameAtom::from("f64vec3"),
                        TypeNameAtom::from("f64vec4"),
                        TypeNameAtom::from("f64mat2"),
                        TypeNameAtom::from("f64mat3"),
                        TypeNameAtom::from("f64mat4"),
                        TypeNameAtom::from("f64mat2x2"),
                        TypeNameAtom::from("f64mat2x3"),
                        TypeNameAtom::from("f64mat2x4"),
                        TypeNameAtom::from("f64mat3x2"),
                        TypeNameAtom::from("f64mat3x3"),
                        TypeNameAtom::from("f64mat3x4"),
                        TypeNameAtom::from("f64mat4x2"),
                        TypeNameAtom::from("f64mat4x3"),
                        TypeNameAtom::from("f64mat4x4"),
                        TypeNameAtom::from("float32_t"),
                        TypeNameAtom::from("f32vec2"),
                        TypeNameAtom::from("f32vec3"),
                        TypeNameAtom::from("f32vec4"),
                        TypeNameAtom::from("f32mat2"),
                        TypeNameAtom::from("f32mat3"),
                        TypeNameAtom::from("f32mat4"),
                        TypeNameAtom::from("f32mat2x2"),
                        TypeNameAtom::from("f32mat2x3"),
                        TypeNameAtom::from("f32mat2x4"),
                        TypeNameAtom::from("f32mat3x2"),
                        TypeNameAtom::from("f32mat3x3"),
                        TypeNameAtom::from("f32mat3x4"),
                        TypeNameAtom::from("f32mat4x2"),
                        TypeNameAtom::from("f32mat4x3"),
                        TypeNameAtom::from("f32mat4x4"),
                        TypeNameAtom::from("float16_t"),
                        TypeNameAtom::from("f16vec2"),
                        TypeNameAtom::from("f16vec3"),
                        TypeNameAtom::from("f16vec4"),
                        TypeNameAtom::from("f16mat2"),
                        TypeNameAtom::from("f16mat3"),
                        TypeNameAtom::from("f16mat4"),
                        TypeNameAtom::from("f16mat2x2"),
                        TypeNameAtom::from("f16mat2x3"),
                        TypeNameAtom::from("f16mat2x4"),
                        TypeNameAtom::from("f16mat3x2"),
                        TypeNameAtom::from("f16mat3x3"),
                        TypeNameAtom::from("f16mat3x4"),
                        TypeNameAtom::from("f16mat4x2"),
                        TypeNameAtom::from("f16mat4x3"),
                        TypeNameAtom::from("f16mat4x4"),
                        TypeNameAtom::from("int64_t"),
                        TypeNameAtom::from("i64vec2"),
                        TypeNameAtom::from("i64vec3"),
                        TypeNameAtom::from("i64vec4"),
                        TypeNameAtom::from("uint64_t"),
                        TypeNameAtom::from("u64vec2"),
                        TypeNameAtom::from("u64vec3"),
                        TypeNameAtom::from("u64vec4"),
                        TypeNameAtom::from("int32_t"),
                        TypeNameAtom::from("i32vec2"),
                        TypeNameAtom::from("i32vec3"),
                        TypeNameAtom::from("i32vec4"),
                        TypeNameAtom::from("uint32_t"),
                        TypeNameAtom::from("u32vec2"),
                        TypeNameAtom::from("u32vec3"),
                        TypeNameAtom::from("u32vec4"),
                        TypeNameAtom::from("int16_t"),
                        TypeNameAtom::from("i16vec2"),
                        TypeNameAtom::from("i16vec3"),
                        TypeNameAtom::from("i16vec4"),
                        TypeNameAtom::from("uint16_t"),
                        TypeNameAtom::from("u16vec2"),
                        TypeNameAtom::from("u16vec3"),
                        TypeNameAtom::from("u16vec4"),
                        TypeNameAtom::from("int8_t"),
                        TypeNameAtom::from("i8vec2"),
                        TypeNameAtom::from("i8vec3"),
                        TypeNameAtom::from("i8vec4"),
                        TypeNameAtom::from("uint8_t"),
                        TypeNameAtom::from("u8vec2"),
                        TypeNameAtom::from("u8vec3"),
                        TypeNameAtom::from("u8vec4"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_explicit_arithmetic_types_float16"),
                    vec![
                        TypeNameAtom::from("float16_t"),
                        TypeNameAtom::from("f16vec2"),
                        TypeNameAtom::from("f16vec3"),
                        TypeNameAtom::from("f16vec4"),
                        TypeNameAtom::from("f16mat2"),
                        TypeNameAtom::from("f16mat3"),
                        TypeNameAtom::from("f16mat4"),
                        TypeNameAtom::from("f16mat2x2"),
                        TypeNameAtom::from("f16mat2x3"),
                        TypeNameAtom::from("f16mat2x4"),
                        TypeNameAtom::from("f16mat3x2"),
                        TypeNameAtom::from("f16mat3x3"),
                        TypeNameAtom::from("f16mat3x4"),
                        TypeNameAtom::from("f16mat4x2"),
                        TypeNameAtom::from("f16mat4x3"),
                        TypeNameAtom::from("f16mat4x4"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_explicit_arithmetic_types_float32"),
                    vec![
                        TypeNameAtom::from("float32_t"),
                        TypeNameAtom::from("f32vec2"),
                        TypeNameAtom::from("f32vec3"),
                        TypeNameAtom::from("f32vec4"),
                        TypeNameAtom::from("f32mat2"),
                        TypeNameAtom::from("f32mat3"),
                        TypeNameAtom::from("f32mat4"),
                        TypeNameAtom::from("f32mat2x2"),
                        TypeNameAtom::from("f32mat2x3"),
                        TypeNameAtom::from("f32mat2x4"),
                        TypeNameAtom::from("f32mat3x2"),
                        TypeNameAtom::from("f32mat3x3"),
                        TypeNameAtom::from("f32mat3x4"),
                        TypeNameAtom::from("f32mat4x2"),
                        TypeNameAtom::from("f32mat4x3"),
                        TypeNameAtom::from("f32mat4x4"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_explicit_arithmetic_types_float64"),
                    vec![
                        TypeNameAtom::from("float64_t"),
                        TypeNameAtom::from("f64vec2"),
                        TypeNameAtom::from("f64vec3"),
                        TypeNameAtom::from("f64vec4"),
                        TypeNameAtom::from("f64mat2"),
                        TypeNameAtom::from("f64mat3"),
                        TypeNameAtom::from("f64mat4"),
                        TypeNameAtom::from("f64mat2x2"),
                        TypeNameAtom::from("f64mat2x3"),
                        TypeNameAtom::from("f64mat2x4"),
                        TypeNameAtom::from("f64mat3x2"),
                        TypeNameAtom::from("f64mat3x3"),
                        TypeNameAtom::from("f64mat3x4"),
                        TypeNameAtom::from("f64mat4x2"),
                        TypeNameAtom::from("f64mat4x3"),
                        TypeNameAtom::from("f64mat4x4"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_explicit_arithmetic_types_int16"),
                    vec![
                        TypeNameAtom::from("int16_t"),
                        TypeNameAtom::from("i16vec2"),
                        TypeNameAtom::from("i16vec3"),
                        TypeNameAtom::from("i16vec4"),
                        TypeNameAtom::from("uint16_t"),
                        TypeNameAtom::from("u16vec2"),
                        TypeNameAtom::from("u16vec3"),
                        TypeNameAtom::from("u16vec4"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_explicit_arithmetic_types_int32"),
                    vec![
                        TypeNameAtom::from("int32_t"),
                        TypeNameAtom::from("i32vec2"),
                        TypeNameAtom::from("i32vec3"),
                        TypeNameAtom::from("i32vec4"),
                        TypeNameAtom::from("uint32_t"),
                        TypeNameAtom::from("u32vec2"),
                        TypeNameAtom::from("u32vec3"),
                        TypeNameAtom::from("u32vec4"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_explicit_arithmetic_types_int64"),
                    vec![
                        TypeNameAtom::from("int64_t"),
                        TypeNameAtom::from("i64vec2"),
                        TypeNameAtom::from("i64vec3"),
                        TypeNameAtom::from("i64vec4"),
                        TypeNameAtom::from("uint64_t"),
                        TypeNameAtom::from("u64vec2"),
                        TypeNameAtom::from("u64vec3"),
                        TypeNameAtom::from("u64vec4"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_explicit_arithmetic_types_int8"),
                    vec![
                        TypeNameAtom::from("int8_t"),
                        TypeNameAtom::from("i8vec2"),
                        TypeNameAtom::from("i8vec3"),
                        TypeNameAtom::from("i8vec4"),
                        TypeNameAtom::from("uint8_t"),
                        TypeNameAtom::from("u8vec2"),
                        TypeNameAtom::from("u8vec3"),
                        TypeNameAtom::from("u8vec4"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_implicit_conversions"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_shader_integer_mix"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_shader_io_blocks"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_non_constant_global_initializers"),
                    vec![],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_shader_texture_image_samples"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_shader_texture_lod"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_terminate_invocation"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_EXT_tessellation_shader"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_texture_buffer"),
                    vec![
                        // GLSL-specific buffer sampler and image types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_EXT_texture_cube_map_array"),
                    vec![
                        // GLSL-specific cube array sampler and image types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(ext_name!("GL_GOOGLE_include_directive"), vec![]),
                ExtensionSpec::new(ext_name!("GL_GOOGLE_cpp_style_line_directive"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_KHR_blend_equation_advanced"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_KHR_shader_subgroup_arithmetic"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_KHR_shader_subgroup_ballot"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_KHR_shader_subgroup_basic"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_KHR_shader_subgroup_clustered"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_KHR_shader_subgroup_quad"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_KHR_shader_subgroup_shuffle"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_KHR_shader_subgroup_shuffle_relative"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_KHR_shader_subgroup_vote"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_NV_mesh_shader"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_NV_ray_tracing"),
                    vec![
                        TypeNameAtom::from("accelerationStructureNV"),
                        TypeNameAtom::from("rayPayloadNV"),
                        TypeNameAtom::from("rayPayloadInNV"),
                        TypeNameAtom::from("hitAttributeNV"),
                        TypeNameAtom::from("callableDataNV"),
                        TypeNameAtom::from("callableDataInNV"),
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_NV_shader_noperspective_interpolation"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_NV_shader_sm_builtins"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_NV_shader_subgroup_partitioned"),
                    vec![],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_OES_EGL_image_external"),
                    vec![TypeNameAtom::from("samplerExternalOES")],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_OES_EGL_image_external_essl3"),
                    vec![TypeNameAtom::from("samplerExternalOES")],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_geometry_point_size"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_geometry_shader"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_gpu_shader5"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_primitive_bounding_box"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_sample_variables"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_shader_image_atomic"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_shader_io_blocks"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_OES_shader_multisample_interpolation"),
                    vec![],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_standard_derivatives"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_tessellation_point_size"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OES_tessellation_shader"), vec![]),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_OES_texture_buffer"),
                    vec![
                        // GLSL-specific buffer sampler and image types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_OES_texture_cube_map_array"),
                    vec![
                        // GLSL-specific cube array sampler and image types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_OES_texture_3D"),
                    vec![
                        // GLSL-specific 3D sampler type not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(
                    ExtNameAtom::from("GL_OES_texture_storage_multisample_2d_array"),
                    vec![
                        // GLSL-specific multisample array sampler types not applicable to HLSL
                    ],
                ),
                ExtensionSpec::new(ExtNameAtom::from("GL_OVR_multiview"), vec![]),
                ExtensionSpec::new(ExtNameAtom::from("GL_OVR_multiview2"), vec![]),
            ]
            .into_iter()
            .map(|spec| (spec.name.clone(), spec))
            .collect(),
        }
    }
}

pub static DEFAULT_REGISTRY: Lazy<Registry> = Lazy::new(Registry::default);
pub static EMPTY_REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);
